import { readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
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
  parseOptions,
} from '../index.js'

const rawSalt = Buffer.from('0123456789abcdef')
const fixture = <T>(name: string): T =>
  JSON.parse(readFileSync(new URL(`./fixtures/${name}.json`, import.meta.url), 'utf8'))
const view = (bytes: Uint8Array) => Uint8Array.from([99, ...bytes, 100]).subarray(1, bytes.length + 1)
const invalidType = { instanceOf: TypeError, code: 'ERR_INVALID_ARG_TYPE' }
const outOfRange = { instanceOf: RangeError, code: 'ERR_OUT_OF_RANGE' }
// What the encoded text itself says, to compare against the parser's reading.
const spelled = (encoded: string) => ({ version: encoded.slice(1, 3), cost: Number(encoded.slice(4, 6)) })

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

test('raw salts use the requested version', async (t) => {
  for (const version of ['2a', '2b', '2y'] as const) {
    const expected = bcryptjs.hashSync('password', `$${version}$04$KBCwKxOzLha2MUDgW0PjXe`)
    t.is(hashSync('password', { cost: 4, salt: rawSalt, version }), expected)
    t.is(await hash('password', { cost: 4, salt: rawSalt, version }), expected)
  }
})

test('wrong option types throw TypeError while bad values throw RangeError', async (t) => {
  for (const options of [{ cost: '10' }, { cost: null }, { version: 2 }, { version: null }]) {
    t.throws(() => genSaltSync(options as never), invalidType)
    t.throws(() => hashSync('password', options as never), invalidType)
    await t.throwsAsync(genSalt(options as never), invalidType)
    await t.throwsAsync(hash('password', options as never), invalidType)
  }
  // @ts-expect-error Exercise an unsupported version string.
  t.throws(() => hashSync('password', { version: '2c' }), outOfRange)
})

test('creation validates costs before integer conversion', async (t) => {
  for (const cost of [3, 32, 4.9, 4294967300, -4294967292, NaN, Infinity, -Infinity]) {
    t.throws(() => genSaltSync({ cost }), outOfRange)
    t.throws(() => hashSync('password', { cost }), outOfRange)
    await t.throwsAsync(genSalt({ cost }), outOfRange)
    await t.throwsAsync(hash('password', { cost }), outOfRange)
  }
  // Validate the upper mathematical bound without computing a cost-31 hash.
  t.true(genSaltSync({ cost: 31 }).startsWith('$2b$31$'))
})

test('creation requires exact raw or canonical encoded salts', async (t) => {
  for (const length of [0, 15, 17]) {
    t.throws(() => hashSync('password', { cost: 4, salt: new Uint8Array(length) }), outOfRange)
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
    // Native argument errors surface with the same class and code as JavaScript validation.
    t.throws(() => hashSync('password', { salt: invalid }), outOfRange)
    await t.throwsAsync(hash('password', { salt: invalid }), outOfRange)
  }
  // @ts-expect-error Encoded salts cannot be combined with cost overrides.
  t.throws(() => hashSync('password', { salt, cost: 4 }), outOfRange)
  // @ts-expect-error Encoded salts cannot be combined with version overrides.
  await t.throwsAsync(hash('password', { salt, version: '2b' }), outOfRange)
  // @ts-expect-error 2x generation is removed.
  await t.throwsAsync(genSalt({ version: '2x' }), outOfRange)
})

test('removed call shapes fail clearly and async validation always rejects', async (t) => {
  // @ts-expect-error Positional calls are intentionally removed.
  t.throws(() => hashSync('password', 4, rawSalt), invalidType)
  // @ts-expect-error Positional calls are intentionally removed.
  const invalid = hash('password', 4, rawSalt)
  t.true(invalid instanceof Promise)
  await t.throwsAsync(invalid, invalidType)
  // @ts-expect-error Positional generator arguments are removed.
  await t.throwsAsync(genSalt(4), invalidType)
  // @ts-expect-error A bare signal must not silently become empty options.
  await t.throwsAsync(verify('password', 'hash', new AbortController().signal), invalidType)
  // @ts-expect-error No verification salt override.
  await t.throwsAsync(verify('password', 'hash', { salt: rawSalt }), invalidType)
  // @ts-expect-error Invalid runtime input must reject instead of throwing before a Promise.
  await t.throwsAsync(hash(null), invalidType)
  // @ts-expect-error Unknown keys must not silently choose the default cost.
  await t.throwsAsync(hash('password', { rounds: 4 }), invalidType)
})

test('default truncation is preserved; strict creation accepts exactly 72 bytes', async (t) => {
  for (const password of ['a'.repeat(71), 'a'.repeat(72), 'é'.repeat(36)]) {
    const result = hashSync(password, { cost: 4, salt: rawSalt, rejectLongPasswords: true })
    t.true(verifySync(password, result))
    t.is(await hash(password, { cost: 4, salt: rawSalt, rejectLongPasswords: true }), result)
  }
  for (const password of ['a'.repeat(73), 'é'.repeat(37)]) {
    t.throws(() => hashSync(password, { cost: 4, rejectLongPasswords: true }), outOfRange)
    await t.throwsAsync(hash(password, { cost: 4, rejectLongPasswords: true }), outOfRange)
    const old = previous.hashSync(password, 4)
    t.true(await verify(password, old))
    const rehashed = await hash(password, { cost: 4 })
    t.true(previous.verifySync(password, rehashed))
    t.true(await verify(password, rehashed))
  }
  const result = hashSync('a'.repeat(72), { cost: 4 })
  t.true(await verify('a'.repeat(72) + 'different suffix', result))
})

test('async work keeps only the password bytes bcrypt reads', async (t) => {
  const long = Buffer.alloc(200, 'a')
  long[150] = 0x62
  const expected = hashSync(long, { cost: 4, salt: rawSalt })
  t.is(expected, bcryptjs.hashSync('a'.repeat(72), '$2b$04$KBCwKxOzLha2MUDgW0PjXe'))
  t.is(await hash(view(long), { cost: 4, salt: rawSalt }), expected)
  t.true(await verify(view(long), expected))
  t.true(await verify(Buffer.alloc(72, 'a'), expected))
  t.false(await verify(Buffer.alloc(71, 'a'), expected))
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
    t.deepEqual(parseOptions(row.hash), spelled(row.hash), label)
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
    // Every hash the verifier accepts is parseable; this includes imported 2x labels.
    if (row.expected) t.deepEqual(parseOptions(row.hash), spelled(row.hash), row.name)
  }
  const parser = fixture<{ fixtures: { name: string; password: string; hash: string; expected: boolean }[] }>(
    'verification-parser-fixtures',
  )
  for (const row of parser.fixtures) {
    t.is(verifySync(row.password, row.hash), row.expected, row.name)
    t.is(await verify(row.password, Buffer.from(row.hash)), row.expected, row.name)
    t.deepEqual(parseOptions(row.hash), { version: '2b', cost: 4 }, row.name)
  }
  t.false(verifySync('password', Buffer.from([255])))
  t.false(await verify('password', Buffer.from([255])))
})

test('parseOptions reads stored hashes with the verifier parser, never the creation parser', async (t) => {
  for (const version of ['2a', '2b', '2y'] as const) {
    for (const cost of [4, 5]) {
      t.deepEqual(parseOptions(hashSync('password', { cost, version })), { version, cost })
    }
  }
  const encoded = await hash('password', { cost: 4 })
  t.deepEqual(parseOptions(Buffer.from(encoded)), { version: '2b', cost: 4 })
  t.deepEqual(parseOptions(view(Buffer.from(encoded))), { version: '2b', cost: 4 })
  // Noncanonical spellings the verifier accepts are reported as their effective values.
  t.deepEqual(parseOptions('$2b$+4$KBCwKxOzLha2MUDgW0PjXeXFrSeJ6fhvcoWu3XdffwQs4TbDlPt/S'), { version: '2b', cost: 4 })
  t.deepEqual(parseOptions('$2x$04$KBCwKxOzLha2MUDgW0PjXeXFrSeJ6fhvcoWu3XdffwQs4TbDlPt/S'), { version: '2x', cost: 4 })
  // Hashes verify can never accept are errors rather than unusable parameters.
  for (const invalid of [
    '',
    'not-a-hash',
    encoded.slice(0, 59),
    `${encoded}a`,
    `$2c${encoded.slice(3)}`,
    `$2b$03${encoded.slice(6)}`,
    `$2b$32${encoded.slice(6)}`,
    // 60 bytes, but not ASCII.
    `${encoded.slice(0, 58)}é`,
    Buffer.from([255]),
  ]) {
    t.throws(() => parseOptions(invalid), outOfRange)
    t.false(verifySync('password', invalid))
  }
  // @ts-expect-error Wrong input types are TypeErrors, like everywhere else.
  t.throws(() => parseOptions(42), invalidType)
  // @ts-expect-error There are no options to pass.
  t.throws(() => parseOptions(encoded, {}), { ...invalidType, message: /single hash argument/ })
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
    await t.throwsAsync(operation(), { name: 'AbortError', code: 'ABORT_ERR' })
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

test('only the package root and package.json are exported', (t) => {
  const require = createRequire(import.meta.url)
  t.is(require('@node-rs/bcrypt').verify, verify)
  t.is(require('@node-rs/bcrypt').parseOptions, parseOptions)
  t.is(require('@node-rs/bcrypt/package.json').name, '@node-rs/bcrypt')
  for (const internal of ['@node-rs/bcrypt/binding', '@node-rs/bcrypt/binding.js', '@node-rs/bcrypt/api.cjs']) {
    t.throws(() => require(internal), { code: 'ERR_PACKAGE_PATH_NOT_EXPORTED' })
  }
})
