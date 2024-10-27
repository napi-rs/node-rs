import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

import { Bench } from 'tinybench'
import chalk from 'chalk'
// @ts-expect-error
import createWasmHasher from 'webpack/lib/util/hash/xxhash64.js'
// @ts-expect-error
import xxhash from 'xxhash'
import xxhashjs from 'xxhashjs'

import { xxh32, xxh64, Xxh32, Xxh64 } from '../index.js'

const FX = readFileSync(join(fileURLToPath(import.meta.url), '..', '..', '..', '..', 'yarn.lock'))

const wasmHasher = createWasmHasher()

const suite = new Bench()

await suite
  .add('@node-rs/xxhash h32', () => {
    xxh32(FX, 0)
  })
  .add('xxhash c++', () => {
    xxhash.hash(FX, 0)
  })
  .add('xxhashjs h32', () => {
    xxhashjs.h32(FX, 0).toNumber()
  })
  .warmup()

console.info(chalk.green('xxh32 without initial seed'))

await suite.run()
console.table(suite.table())

const multiStepSuite = new Bench()

await multiStepSuite
  .add('@node-rs/xxhash h32', () => {
    new Xxh32().update(FX).digest()
  })
  .add('xxhashjs h32', () => {
    xxhashjs.h32().update(FX).digest().toNumber()
  })
  .warmup()

await multiStepSuite.run()

console.info(chalk.green('xxh32 without initial seed multi step'))
console.table(multiStepSuite.table())

const xx64Suite = new Bench()

await xx64Suite
  .add('@node-rs/xxhash 64', () => {
    xxh64(FX).toString(16)
  })
  .add('xxhash C++', () => {
    xxhash.hash64(FX, 0)
  })
  .add('wasm', () => {
    wasmHasher.update(FX).digest()
    wasmHasher.reset()
  })
  .add('xxhashjs h64', () => {
    xxhashjs.h64(FX, 0).toString(16)
  })
  .warmup()

await xx64Suite.run()

console.info(chalk.green('xxh64 without initial seed'))
console.table(xx64Suite.table())

const multiStepSuite64 = new Bench()

await multiStepSuite64
  .add('@node-rs/xxhash 64', () => {
    new Xxh64().update(FX).digest().toString(16)
  })
  .add('wasm', () => {
    wasmHasher.update(FX).digest()
    wasmHasher.reset()
  })
  .add('xxhashjs h64', () => {
    xxhashjs.h64(0).update(FX).digest().toString(16)
  })
  .warmup()

await multiStepSuite64.run()

console.info(chalk.green('xxh64 without initial seed multi step'))
console.table(multiStepSuite64.table())
