import { createRequire } from 'node:module'
import test from 'ava'
import type * as API from '../index.js'

const createBcrypt = createRequire(import.meta.url)('../api.cjs') as (binding: Record<string, unknown>) => typeof API

type Pending = { signal: AbortSignal; resolve: (value: string) => void; reject: (error: Error) => void }
function controlled() {
  const pending: Pending[] = []
  const api = createBcrypt({
    BCRYPT_API_VERSION: 2,
    DEFAULT_COST: 12,
    hash: (...args: unknown[]) =>
      new Promise<string>((resolve, reject) => pending.push({ signal: args[5] as AbortSignal, resolve, reject })),
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
  pending[1].resolve('discarded')
  t.is(await completed, 'completed')
})
