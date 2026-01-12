import { cpus } from 'node:os'

import nodeArgon2 from 'argon2'
import { Bench } from 'tinybench'

import { hash, verify, Algorithm } from '../index.js'

const PASSWORD = 'test-password-for-benchmark'
const CORES = cpus().length

// Use aligned parameters for fair comparison
// These are OWASP-recommended settings for Argon2id
const MEMORY_COST = 65536 // 64 MiB
const TIME_COST = 3 // 3 iterations
const PARALLELISM = CORES

const HASHED = await hash(PASSWORD, {
  algorithm: Algorithm.Argon2id,
  memoryCost: MEMORY_COST,
  timeCost: TIME_COST,
  parallelism: PARALLELISM,
})

const bench = new Bench()

bench
  .add('@node-rs/argon2 hash', async () => {
    await hash(PASSWORD, {
      algorithm: Algorithm.Argon2id,
      memoryCost: MEMORY_COST,
      timeCost: TIME_COST,
      parallelism: PARALLELISM,
    })
  })
  .add('node-argon2 hash', async () => {
    await nodeArgon2.hash(PASSWORD, {
      type: nodeArgon2.argon2id,
      memoryCost: MEMORY_COST,
      timeCost: TIME_COST,
      parallelism: PARALLELISM,
    })
  })
  .add('@node-rs/argon2 verify', async () => {
    console.assert(await verify(HASHED, PASSWORD))
  })
  .add('node-argon2 verify', async () => {
    console.assert(await nodeArgon2.verify(HASHED, PASSWORD))
  })

await bench.run()

console.table(bench.table())
