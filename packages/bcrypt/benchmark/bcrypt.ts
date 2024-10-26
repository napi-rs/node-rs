import openbsd from '@cwasm/openbsd-bcrypt'
import openwall from '@cwasm/openwall-bcrypt'
import { hashSync, compare, genSaltSync } from 'bcrypt'
import bcryptjs from 'bcryptjs'
import { Bench } from 'tinybench'
import chalk from 'chalk'

import { hashSync as napiHashSync, verifySync, genSaltSync as napiGenSaltSync } from '../binding.js'

const password = 'node-rust-password'

const syncHashSuite = new Bench()
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

await syncHashSuite.warmup()
await syncHashSuite.run()

console.info(chalk.green('Hash benchmark'))
console.table(syncHashSuite.table())

const verifySuite = new Bench()
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

await verifySuite.warmup()
await verifySuite.run()

console.info(chalk.green('Verify benchmark'))
console.table(verifySuite.table())

const genSaltSuite = new Bench()
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

await genSaltSuite.warmup()
await genSaltSuite.run()

console.info(chalk.green('GenSalt benchmark'))
console.table(genSaltSuite.table())
