import test from 'ava'

import { verifySync, compareSync, verify, compare, hash, genSaltSync } from '../index'

const { hashSync } = require('bcryptjs')

const fx = Buffer.from('bcrypt-test-password')

const hashedPassword = hashSync(fx.toString('utf8'), genSaltSync())

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
