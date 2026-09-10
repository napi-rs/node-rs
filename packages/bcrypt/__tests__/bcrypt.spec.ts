import { readFileSync } from 'node:fs'
import test from 'ava'
import bcryptjs from 'bcryptjs'
import previous from 'bcrypt-previous'
import {
  DEFAULT_COST,
  genSalt,
  genSaltSync,
  hash,
  hashSync,
  verify,
  verifySync,
  compare,
  compareSync,
} from '../index.js'

const rawSalt = Buffer.from('0123456789abcdef')
const fixture = <T>(name: string): T =>
  JSON.parse(readFileSync(new URL(`./fixtures/${name}.json`, import.meta.url), 'utf8'))
const view = (bytes: Uint8Array) => Uint8Array.from([99, ...bytes, 100]).subarray(1, bytes.length + 1)

test('generated salts compose with hashing and independent implementations', async (t) => {
  t.is(DEFAULT_COST, 12)
  t.regex(genSaltSync(), /^\$2b\$12\$/)
  for (const version of ['2a', '2b', '2y'] as const) {
    for (const salt of [genSaltSync({ cost: 4, version }), await genSalt({ cost: 4, version })]) {
      t.is(salt.length, 29)
      t.true(salt.startsWith(`$${version}$04$`))
      const expected = bcryptjs.hashSync('password', salt)
      t.is(hashSync('password', { salt }), expected)
      t.is(await hash('password', { salt }), expected)
      t.true(await previous.verify('password', expected))
      t.false(previous.verifySync('wrong', expected))
    }
  }
  t.is(hashSync('password', { cost: 4, salt: rawSalt }), bcryptjs.hashSync('password', '$2b$04$KBCwKxOzLha2MUDgW0PjXe'))
})

test('creation validates costs before integer conversion', async (t) => {
  for (const cost of [3, 32, 4.9, 4294967300, -4294967292, NaN, Infinity, -Infinity]) {
    t.throws(() => genSaltSync({ cost }), { instanceOf: RangeError })
    t.throws(() => hashSync('password', { cost }), { instanceOf: RangeError })
    await t.throwsAsync(genSalt({ cost }), { instanceOf: RangeError })
    await t.throwsAsync(hash('password', { cost }), { instanceOf: RangeError })
  }
  // Validate the upper mathematical bound without computing a cost-31 hash.
  t.true(genSaltSync({ cost: 31 }).startsWith('$2b$31$'))
})

test('creation requires exact raw or canonical encoded salts', async (t) => {
  for (const length of [0, 15, 17]) {
    t.throws(() => hashSync('password', { cost: 4, salt: new Uint8Array(length) }), { instanceOf: RangeError })
  }
  const salt = '$2b$04$KBCwKxOzLha2MUDgW0PjXe'
  for (const invalid of [
    '',
    'hello',
    `${salt}==`,
    salt.replace('2b', '2x'),
    salt.replace('04', '+4'),
    salt.slice(0, -1) + 'f',
    salt.replace('K', 'é'),
  ]) {
    t.throws(() => hashSync('password', { salt: invalid }), { instanceOf: RangeError })
    await t.throwsAsync(hash('password', { salt: invalid }), { instanceOf: RangeError })
  }
  // @ts-expect-error Encoded salts cannot be combined with cost overrides.
  t.throws(() => hashSync('password', { salt, cost: 4 }), { instanceOf: RangeError })
  // @ts-expect-error Encoded salts cannot be combined with version overrides.
  await t.throwsAsync(hash('password', { salt, version: '2b' }), { instanceOf: RangeError })
  // @ts-expect-error 2x generation is removed.
  await t.throwsAsync(genSalt({ version: '2x' }), { instanceOf: RangeError })
})

test('removed call shapes fail clearly and async validation always rejects', async (t) => {
  // @ts-expect-error Positional calls are intentionally removed.
  t.throws(() => hashSync('password', 4, rawSalt), { instanceOf: TypeError })
  // @ts-expect-error Positional calls are intentionally removed.
  const invalid = hash('password', 4, rawSalt)
  t.true(invalid instanceof Promise)
  await t.throwsAsync(invalid, { instanceOf: TypeError })
  // @ts-expect-error Positional generator arguments are removed.
  await t.throwsAsync(genSalt(4), { instanceOf: TypeError })
  // @ts-expect-error A bare signal must not silently become empty options.
  await t.throwsAsync(verify('password', 'hash', new AbortController().signal), { instanceOf: TypeError })
  // @ts-expect-error No verification salt override.
  await t.throwsAsync(verify('password', 'hash', { salt: rawSalt }), { instanceOf: TypeError })
  // @ts-expect-error Invalid runtime input must reject instead of throwing before a Promise.
  await t.throwsAsync(hash(null), { instanceOf: TypeError })
  // @ts-expect-error Unknown keys must not silently choose the default cost.
  await t.throwsAsync(hash('password', { rounds: 4 }), { instanceOf: TypeError })
})

test('default truncation is preserved; strict creation accepts exactly 72 bytes', async (t) => {
  for (const password of ['a'.repeat(71), 'a'.repeat(72), 'é'.repeat(36)]) {
    const result = hashSync(password, { cost: 4, salt: rawSalt, rejectLongPasswords: true })
    t.true(verifySync(password, result))
    t.is(await hash(password, { cost: 4, salt: rawSalt, rejectLongPasswords: true }), result)
  }
  for (const password of ['a'.repeat(73), 'é'.repeat(37)]) {
    t.throws(() => hashSync(password, { cost: 4, rejectLongPasswords: true }), { instanceOf: RangeError })
    await t.throwsAsync(hash(password, { cost: 4, rejectLongPasswords: true }), { instanceOf: RangeError })
    const old = previous.hashSync(password, 4)
    t.true(await verify(password, old))
    const rehashed = await hash(password, { cost: 4 })
    t.true(previous.verifySync(password, rehashed))
    t.true(await verify(password, rehashed))
  }
  const result = hashSync('a'.repeat(72), { cost: 4 })
  t.true(await verify('a'.repeat(72) + 'different suffix', result))
})

test('published historical hashes retain authentication and byte view handling', async (t) => {
  const { fixtures } = fixture<{
    fixtures: { generatorVersion: string; passwordText?: string; passwordHex: string; hash: string }[]
  }>('historical-hash-fixtures')
  for (const row of fixtures) {
    const bytes = Buffer.from(row.passwordHex, 'hex')
    const password = row.passwordText ?? bytes
    const label = `${row.generatorVersion}: ${row.passwordHex}`
    t.true(verifySync(password, row.hash), label)
    t.true(await verify(password, row.hash), label)
    t.true(verifySync(view(bytes), view(Buffer.from(row.hash))), label)
    t.true(await verify(view(bytes), view(Buffer.from(row.hash))), label)
    t.false(await verify(Buffer.concat([Buffer.from('!'), bytes]), row.hash), label)
  }
})

test('previous-release acceptance and rejection outcomes stay frozen', async (t) => {
  const { fixtures } = fixture<{ fixtures: { name: string; passwordHex: string; hash: string; expected: boolean }[] }>(
    'stored-hash-fixtures',
  )
  for (const row of fixtures) {
    const password = Buffer.from(row.passwordHex, 'hex')
    t.is(verifySync(password, row.hash), row.expected, row.name)
    t.is(await verify(password, row.hash), row.expected, row.name)
  }
  const parser = fixture<{ fixtures: { name: string; password: string; hash: string; expected: boolean }[] }>(
    'verification-parser-fixtures',
  )
  for (const row of parser.fixtures) {
    t.is(verifySync(row.password, row.hash), row.expected, row.name)
    t.is(await verify(row.password, Buffer.from(row.hash)), row.expected, row.name)
  }
  t.false(verifySync('password', Buffer.from([255])))
  t.false(await verify('password', Buffer.from([255])))
})

test('async calls own password, salt, and stored-hash bytes before returning', async (t) => {
  const password = view(Buffer.from('original'))
  const salt = view(rawSalt)
  const expected = hashSync('original', { cost: 4, salt: rawSalt })
  const pending = hash(password, { cost: 4, salt })
  password.fill(33)
  salt.fill(0)
  t.is(await pending, expected)
  const input = view(Buffer.from('original'))
  const encoded = view(Buffer.from(expected))
  const checking = verify(input, encoded)
  input.fill(33)
  encoded.fill(33)
  t.true(await checking)
})

test('pre-aborted and reused signals reject without overwriting handlers', async (t) => {
  const stopped = new AbortController()
  stopped.abort()
  for (const operation of [
    () => genSalt({ cost: 4, signal: stopped.signal }),
    () => hash('password', { cost: 4, signal: stopped.signal }),
    () => verify('password', 'hash', { signal: stopped.signal }),
  ]) {
    await t.throwsAsync(operation(), { name: 'AbortError' })
  }
  const controller = new AbortController()
  let propertyCalls = 0
  let listenerCalls = 0
  controller.signal.onabort = () => propertyCalls++
  controller.signal.addEventListener('abort', () => listenerCalls++)
  await hash('completed', { cost: 4, signal: controller.signal })
  const first = hash('queued one', { cost: 4, signal: controller.signal })
  const second = hash('queued two', { cost: 4, signal: controller.signal })
  controller.abort()
  await t.throwsAsync(first, { name: 'AbortError' })
  await t.throwsAsync(second, { name: 'AbortError' })
  t.is(propertyCalls, 1)
  t.is(listenerCalls, 1)
})

test('comparison aliases and public exports remain consistent', (t) => {
  t.is(compare, verify)
  t.is(compareSync, verifySync)
})
