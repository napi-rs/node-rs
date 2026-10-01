// Run directly on the oldest supported Node versions, without the modern test runner.
const assert = require('assert')
const bcrypt = require('../index.js')
const historical = require('./fixtures/historical-hash-fixtures.json').fixtures
const outcomes = require('./fixtures/stored-hash-fixtures.json').fixtures
const parser = require('./fixtures/verification-parser-fixtures.json').fixtures

async function main() {
  assert.strictEqual(bcrypt.DEFAULT_COST, 12)
  assert.strictEqual(bcrypt.compare, bcrypt.verify)
  assert.strictEqual(bcrypt.compareSync, bcrypt.verifySync)
  // Resolves through "exports" on Node 12 and through "main" on Node 10.
  assert.strictEqual(require('@node-rs/bcrypt').verify, bcrypt.verify)
  assert.strictEqual(require('@node-rs/bcrypt').parseOptions, bcrypt.parseOptions)
  assert.ok(bcrypt.genSaltSync().startsWith('$2b$12$'))
  assert.ok((await bcrypt.genSalt()).startsWith('$2b$12$'))

  for (const version of ['2a', '2b', '2y']) {
    const salt = await bcrypt.genSalt({ cost: 4, version })
    assert.strictEqual(salt.length, 29)
    assert.ok(salt.startsWith(`$${version}$04$`))
    const hash = bcrypt.hashSync('password', { salt })
    assert.strictEqual(await bcrypt.hash('password', { salt }), hash)
    assert.strictEqual(bcrypt.verifySync('password', hash), true)
    assert.strictEqual(await bcrypt.verify('wrong', hash), false)
    assert.deepStrictEqual(bcrypt.parseOptions(hash), { version, cost: 4 })
    assert.deepStrictEqual(bcrypt.parseOptions(Buffer.from(hash)), { version, cost: 4 })
  }
  assert.deepStrictEqual(bcrypt.parseOptions('$2b$+4$KBCwKxOzLha2MUDgW0PjXeXFrSeJ6fhvcoWu3XdffwQs4TbDlPt/S'), {
    version: '2b',
    cost: 4,
  })
  assert.throws(() => bcrypt.parseOptions('not-a-hash'), { name: 'RangeError', code: 'ERR_OUT_OF_RANGE' })
  assert.throws(() => bcrypt.parseOptions(42), { name: 'TypeError', code: 'ERR_INVALID_ARG_TYPE' })

  const password = Buffer.from('original')
  const salt = Buffer.alloc(16)
  const expected = bcrypt.hashSync('original', { cost: 4, salt })
  const pending = bcrypt.hash(password, { cost: 4, salt })
  password.fill(33)
  salt.fill(33)
  assert.strictEqual(await pending, expected)

  const longPassword = 'a'.repeat(73)
  const longHash = await bcrypt.hash(longPassword, { cost: 4 })
  assert.strictEqual(await bcrypt.verify(longPassword, longHash), true)
  const outOfRange = { name: 'RangeError', code: 'ERR_OUT_OF_RANGE' }
  const invalidType = { name: 'TypeError', code: 'ERR_INVALID_ARG_TYPE' }
  assert.throws(() => bcrypt.hashSync(longPassword, { cost: 4, rejectLongPasswords: true }), outOfRange)
  await assert.rejects(bcrypt.hash('password', { salt: 'invalid' }), outOfRange)
  await assert.rejects(bcrypt.genSalt({ cost: 3 }), outOfRange)
  await assert.rejects(bcrypt.genSalt({ cost: '4' }), invalidType)
  const invalid = bcrypt.hash('password', 4)
  assert.ok(invalid instanceof Promise)
  await assert.rejects(invalid, invalidType)
  await assert.rejects(bcrypt.verify('password', expected, { signal: {} }), invalidType)

  for (const row of historical) {
    const input = row.passwordText === undefined ? Buffer.from(row.passwordHex, 'hex') : row.passwordText
    assert.strictEqual(bcrypt.verifySync(input, row.hash), true)
    assert.strictEqual(await bcrypt.verify(input, row.hash), true)
    assert.deepStrictEqual(bcrypt.parseOptions(row.hash), {
      version: row.hash.slice(1, 3),
      cost: Number(row.hash.slice(4, 6)),
    })
  }
  for (const row of outcomes) {
    const input = Buffer.from(row.passwordHex, 'hex')
    assert.strictEqual(bcrypt.verifySync(input, row.hash), row.expected, row.name)
    assert.strictEqual(await bcrypt.verify(input, row.hash), row.expected, row.name)
  }
  for (const row of parser) {
    assert.strictEqual(bcrypt.verifySync(row.password, row.hash), row.expected, row.name)
    assert.strictEqual(await bcrypt.verify(row.password, row.hash), row.expected, row.name)
  }
  console.log(`Bcrypt public API and stored-hash checks passed on ${process.version}`)
}

main().catch((error) => {
  console.error(error)
  process.exitCode = 1
})
