import test from 'ava'

import {
  verifySync,
  compareSync,
  verify,
  compare,
  hash,
  genSaltSync,
  genSalt,
  hashSync as bcryptHashSync,
} from '../index'

const { hashSync } = require('bcryptjs')

const fx = Buffer.from('bcrypt-test-password')

const hashedPassword = hashSync(fx.toString('utf8'), 10)

test('genSaltSync should return a string', (t) => {
  t.is(typeof genSaltSync(10), 'string')
  t.is(typeof genSaltSync(10, '2a'), 'string')
  t.is(typeof genSaltSync(10, '2b'), 'string')
  t.is(typeof genSaltSync(10, '2y'), 'string')
  t.is(typeof genSaltSync(10, '2x'), 'string')
  t.throws(() => genSaltSync(10, 'invalid' as any))
})

test('genSalt should return a string', async (t) => {
  t.is(typeof (await genSalt(10)), 'string')
  t.is(typeof (await genSalt(10, '2a')), 'string')
  t.is(typeof (await genSalt(10, '2b')), 'string')
  t.is(typeof (await genSalt(10, '2y')), 'string')
  t.is(typeof (await genSalt(10, '2x')), 'string')
  t.throws(() => genSalt(10, 'invalid' as any))
})

test('verifySync hashed password from bcrypt should be true', (t) => {
  t.true(verifySync(fx, hashedPassword))
})

test('verifySync hashed password from @node-rs/bcrypt should be true', async (t) => {
  const hashed = await hash(fx)
  t.true(verifySync(fx, hashed))
})

test('verifySync should always return boolean even if the password is invalid', (t) => {
  t.false(verifySync('a', 'b'))
  t.false(verifySync('a', ''))
  t.false(verifySync('', ''))
})

test('compare should be equal to verify', (t) => {
  t.is(verifySync, compareSync)
  t.is(verify, compare)
})

test('hash should support long or short string', (t) => {
  t.is(typeof bcryptHashSync('string', 10, 'hello'), 'string')
  t.is(typeof bcryptHashSync('string', 10, 'aloooooooooooooooooooooongsalt'), 'string')
})
