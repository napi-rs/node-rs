import test from 'ava'

import { verifySync, hash } from '../index'

const { hashSync } = require('bcrypt')

const fx = Buffer.from('bcrypt-test-password')

const hashedPassword = hashSync(fx)

test('verifySync hashed password from bcrypt should be true', (t) => {
  t.true(verifySync(fx, hashedPassword))
})

test('verifySync hashed password from @node-rs/bcrypt should be true', async (t) => {
  const hashed = await hash(fx)
  t.true(verifySync(fx, hashed))
})
