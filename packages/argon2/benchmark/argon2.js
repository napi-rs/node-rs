import { cpus } from 'node:os'

import nodeArgon2 from 'argon2'
import { Bench } from 'tinybench'

import { hash, verify, Algorithm } from '../index.js'

const PASSWORD = '$v=19$m=4096,t=3,p=1$fyLYvmzgpBjDTP6QSypj3g$pb1Q3Urv1amxuFft0rGwKfEuZPhURRDV7TJqcBnwlGo'
const CORES = cpus().length

const HASHED = await hash(PASSWORD, {
  algorithm: Algorithm.Argon2id,
  parallelism: CORES,
})

const bench = new Bench('Hash with all cores')

bench
  .add('@node-rs/argon hash', async () => {
    await hash(PASSWORD, {
      algorithm: Algorithm.Argon2id,
      parallelism: CORES,
    })
  })
  .add('node-argon hash', async () => {
    await nodeArgon2.hash(PASSWORD, { type: nodeArgon2.argon2id, parallelism: CORES })
  })
  .add('@node-rs/argon verify', async () => {
    console.assert(await verify(HASHED, PASSWORD))
  })
  .add('node-argon verify', async () => {
    console.assert(await nodeArgon2.verify(HASHED, PASSWORD))
  })

await bench.warmup()
await bench.run()

console.table(bench.table())
