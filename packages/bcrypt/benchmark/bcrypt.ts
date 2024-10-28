import openbsd from '@cwasm/openbsd-bcrypt'
import openwall from '@cwasm/openwall-bcrypt'
import { hashSync, compare, genSaltSync } from 'bcrypt'
import bcryptjs from 'bcryptjs'
import { Bench } from 'tinybench'

import { hashSync as napiHashSync, verifySync, genSaltSync as napiGenSaltSync } from '../binding.js'

const password = 'node-rust-password'

const syncHashSuite = new Bench({
  name: 'Hash benchmark',
})

syncHashSuite
  .add('@node-rs/bcrypt', () => {
    napiHashSync(password, 10)
  })
  .add('node bcrypt', () => {
    hashSync(password, 10)
  })
  .add('bcryptjs', () => {
    bcryptjs.hashSync(password, 10)
  })
  .add('wasm OpenBSD', () => {
    openbsd.hashSync(password, 10)
  })
  .add('wasm Openwall', () => {
    openwall.hashSync(password, 10)
  })

await syncHashSuite.run()

console.table(syncHashSuite.table())

const verifySuite = new Bench({
  name: 'Verify benchmark`',
})
const hashed = napiHashSync(password, 12)
verifySuite
  .add('@node-rs/bcrypt', () => {
    verifySync(password, hashed)
  })
  .add('node bcrypt', () => {
    compare(password, hashSync(password, 12))
  })
  .add('bcryptjs', () => {
    bcryptjs.compareSync(password, hashed)
  })

await verifySuite.run()

console.table(verifySuite.table())

const genSaltSuite = new Bench({
  name: 'GenSalt benchmark',
})
genSaltSuite
  .add('@node-rs/bcrypt', () => {
    napiGenSaltSync(12)
  })
  .add('node bcrypt', () => {
    genSaltSync(12)
  })
  .add('bcryptjs', () => {
    bcryptjs.genSaltSync(12)
  })
  .add('wasm OpenBSD', () => {
    openbsd.genSaltSync(12)
  })
  .add('wasm Openwall', () => {
    openwall.genSaltSync(12)
  })

await genSaltSuite.run()

console.table(genSaltSuite.table())
