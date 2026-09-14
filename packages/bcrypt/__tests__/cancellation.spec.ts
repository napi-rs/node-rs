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
