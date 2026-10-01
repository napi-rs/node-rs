import { createRequire } from 'node:module'
import test from 'ava'
import { AbortController as PolyfillAbortController } from 'abort-controller'
import * as bcrypt from '../index.js'
import type * as API from '../index.js'

const require = createRequire(import.meta.url)
const createBcrypt = require('../api.cjs') as (binding: Record<string, unknown>) => typeof API
const checkPolyfillCancellation = require('./polyfill-cancellation.cjs') as (api: typeof API) => Promise<void>

type NativeSignal = { aborted: boolean; onabort?: () => void }
type Pending = {
  signal: NativeSignal
  cancellations: number
  resolve: (value: string) => void
  reject: (error: Error) => void
}
function controlled() {
  const pending: Pending[] = []
  const api = createBcrypt({
    BCRYPT_API_VERSION: 2,
    DEFAULT_COST: 12,
    hash: (...args: unknown[]) =>
      new Promise<string>((resolve, reject) => {
        const signal = args[5] as NativeSignal
        const task = { signal, cancellations: 0, resolve, reject }
        signal.onabort = function () {
          if (this !== signal) throw new Error('The native callback requires its original receiver')
          task.cancellations++
        }
        pending.push(task)
      }),
  })
  return { api, pending }
}

// An older EventTarget style: the third argument is only a capture flag, and there is no reason.
function legacySignal() {
  const entries: { listener: () => void; capture: boolean }[] = []
  return {
    aborted: false,
    entries,
    addEventListener(_type: 'abort', listener: () => void, capture?: unknown) {
      entries.push({ listener, capture: Boolean(capture) })
    },
    removeEventListener(_type: 'abort', listener: () => void, capture?: unknown) {
      const index = entries.findIndex((entry) => entry.listener === listener && entry.capture === Boolean(capture))
      if (index !== -1) entries.splice(index, 1)
    },
    abort() {
      this.aborted = true
      for (const { listener } of entries.slice()) listener()
    },
  }
}

test('a mismatched backend cannot silently interpret major-version calls', (t) => {
  t.throws(() => createBcrypt({ DEFAULT_COST: 12 }), { message: /Incompatible bcrypt binary/ })
})

test('aborting running work settles publicly and consumes later native failure', async (t) => {
  const { api, pending } = controlled()
  const controller = new AbortController()
  const operation = api.hash('password', { signal: controller.signal })
  t.is(pending.length, 1)
  controller.abort()
  await t.throwsAsync(operation, { name: 'AbortError' })
  t.true(pending[0].signal.aborted)
  t.is(pending[0].cancellations, 1)
  pending[0].reject(new Error('late native failure'))
  await Promise.resolve()
  t.pass()
})

test('abort wins if native completion has not yet been observed', async (t) => {
  const { api, pending } = controlled()
  const controller = new AbortController()
  const operation = api.hash('password', { signal: controller.signal })
  pending[0].resolve('native result')
  controller.abort()
  await t.throwsAsync(operation, { name: 'AbortError' })
})

test('completion wins once observed, and reused signals get independent native state', async (t) => {
  const { api, pending } = controlled()
  const controller = new AbortController()
  const completed = api.hash('first', { signal: controller.signal })
  pending[0].resolve('completed')
  t.is(await completed, 'completed')
  const next = api.hash('second', { signal: controller.signal })
  t.not(pending[0].signal, pending[1].signal)
  controller.abort()
  await t.throwsAsync(next, { name: 'AbortError' })
  t.false(pending[0].signal.aborted)
  t.true(pending[1].signal.aborted)
  t.is(pending[0].cancellations, 0)
  t.is(pending[1].cancellations, 1)
  pending[1].resolve('discarded')
  t.is(await completed, 'completed')
})

test('locally imported polyfill signals work alongside native globals and match the public types', async (t) => {
  const controller = new PolyfillAbortController()
  const options: API.AsyncOptions = { signal: controller.signal }
  const encoded = bcrypt.hashSync('password', { cost: 4 })
  t.true(await bcrypt.verify('password', encoded, options))
  await checkPolyfillCancellation(bcrypt)
})

test('incomplete signal interfaces reject before calling the native backend', async (t) => {
  const { api, pending } = controlled()
  for (const signal of [null, {}, { aborted: false }, { aborted: false, addEventListener() {} }]) {
    // @ts-expect-error Exercise incomplete cancellation interfaces from JavaScript.
    await t.throwsAsync(api.hash('password', { signal }), { instanceOf: TypeError })
  }
  t.is(pending.length, 0)
})

test('listeners are removed with the options they were added with', async (t) => {
  const { api, pending } = controlled()
  const signal = legacySignal()
  const completed = api.hash('first', { signal })
  t.is(signal.entries.length, 1)
  pending[0].resolve('done')
  t.is(await completed, 'done')
  t.is(signal.entries.length, 0)
  const aborted = api.hash('second', { signal })
  signal.abort()
  const error = await t.throwsAsync(aborted, { name: 'AbortError' })
  t.false('cause' in error!)
  t.is(signal.entries.length, 0)
})

test('a throwing removeEventListener cannot crash a settled call or skip native cancellation', async (t) => {
  const { api, pending } = controlled()
  const signal = legacySignal()
  signal.removeEventListener = () => {
    throw new Error('remove failed')
  }
  const unhandled: unknown[] = []
  const onUnhandled = (reason: unknown) => unhandled.push(reason)
  process.on('unhandledRejection', onUnhandled)
  const completed = api.hash('first', { signal })
  pending[0].resolve('done')
  t.is(await completed, 'done')
  const aborted = api.hash('second', { signal })
  t.notThrows(() => signal.abort())
  await t.throwsAsync(aborted, { name: 'AbortError' })
  t.is(pending[1].cancellations, 1)
  await new Promise((resolve) => setImmediate(resolve))
  process.off('unhandledRejection', onUnhandled)
  t.deepEqual(unhandled, [])
})

test('an unreadable signal reason still aborts with AbortError', async (t) => {
  const { api, pending } = controlled()
  const signal = legacySignal()
  Object.defineProperty(signal, 'reason', {
    get() {
      throw new Error('unreadable')
    },
  })
  const running = api.hash('password', { signal })
  t.notThrows(() => signal.abort())
  const error = await t.throwsAsync(running, { name: 'AbortError' })
  t.false('cause' in error!)
  t.is(pending[0].cancellations, 1)
  const preAborted = await t.throwsAsync(api.hash('password', { signal }), { name: 'AbortError' })
  t.false('cause' in preAborted!)
  t.is(pending.length, 1)
})

test('AbortError keeps the signal reason as a non-enumerable cause', async (t) => {
  const { api } = controlled()
  const reason = new Error('stop')
  const controller = new AbortController()
  const custom = api.hash('password', { signal: controller.signal })
  controller.abort(reason)
  const error = await t.throwsAsync(custom, { name: 'AbortError' })
  t.is(error!.cause, reason)
  t.false(Object.getOwnPropertyDescriptor(error, 'cause')!.enumerable)

  const plain = new AbortController()
  const defaulted = api.hash('password', { signal: plain.signal })
  plain.abort()
  t.is((await t.throwsAsync(defaulted, { name: 'AbortError' }))!.cause, plain.signal.reason)

  // AbortSignal.timeout() does not keep the event loop alive, and the controlled backend
  // has no native work that would; hold the loop open until the timeout fires.
  const keepAlive = setTimeout(() => {}, 10_000)
  const timedOut = await t.throwsAsync(api.hash('password', { signal: AbortSignal.timeout(1) }), { name: 'AbortError' })
  clearTimeout(keepAlive)
  t.is((timedOut!.cause as Error).name, 'TimeoutError')

  const preAborted = await t.throwsAsync(bcrypt.verify('password', 'hash', { signal: AbortSignal.abort(reason) }), {
    name: 'AbortError',
  })
  t.is(preAborted!.cause, reason)
})
