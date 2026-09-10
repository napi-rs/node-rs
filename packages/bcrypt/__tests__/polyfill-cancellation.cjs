const assert = require('assert')
const { AbortController, AbortSignal } = require('abort-controller')

module.exports = async function checkPolyfillCancellation(api) {
  const encoded = api.hashSync('password', { cost: 4 })
  const operations = [
    (signal) => api.genSalt({ cost: 4, signal }),
    (signal) => api.hash('password', { cost: 4, signal }),
    (signal) => api.verify('password', encoded, { signal }),
  ]
  const isAbortError = (error) => error.name === 'AbortError'
  const stopped = new AbortController()
  stopped.abort()
  for (const operation of operations) {
    await assert.rejects(operation(stopped.signal), isAbortError)
  }

  for (const operation of operations) {
    const controller = new AbortController()
    const signal = controller.signal
    let propertyCalls = 0
    let listenerCalls = 0
    const onabort = () => propertyCalls++
    signal.onabort = onabort
    signal.addEventListener('abort', () => listenerCalls++)

    // Track only listeners added by the adapter, independently of the polyfill internals.
    const listeners = new Set()
    const add = signal.addEventListener
    const remove = signal.removeEventListener
    signal.addEventListener = function (type, listener, options) {
      listeners.add(listener)
      return add.call(this, type, listener, options)
    }
    signal.removeEventListener = function (type, listener, options) {
      listeners.delete(listener)
      return remove.call(this, type, listener, options)
    }

    assert.ok(await operation(signal))
    assert.strictEqual(signal.onabort, onabort)
    assert.strictEqual(listeners.size, 0)

    const first = operation(signal)
    const second = operation(signal)
    assert.strictEqual(listeners.size, 2)
    controller.abort()
    await assert.rejects(first, isAbortError)
    await assert.rejects(second, isAbortError)
    assert.strictEqual(signal.onabort, onabort)
    assert.strictEqual(propertyCalls, 1)
    assert.strictEqual(listenerCalls, 1)
    assert.strictEqual(listeners.size, 0)
  }
}

if (require.main === module) {
  const api = require('../index.js')
  // This runs in its own process, including on modern Node versions.
  delete global.AbortController
  delete global.AbortSignal
  module
    .exports(api)
    .then(() => {
      // A partial global shim must not require a matching global controller.
      global.AbortSignal = AbortSignal
      return module.exports(api)
    })
    .then(() => console.log(`Imported polyfill cancellation passed on ${process.version}`))
    .catch((error) => {
      console.error(error)
      process.exitCode = 1
    })
}
