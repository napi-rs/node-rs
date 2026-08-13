import { argon2, argon2Sync } from 'node:crypto'
import { performance } from 'node:perf_hooks'
import { promisify } from 'node:util'

import { argon2id as nobleArgon2id } from '@noble/hashes/argon2.js'
import nodeArgon2 from 'argon2'
import { argon2id as wasmArgon2id } from 'hash-wasm'

import { Algorithm, hashRaw, hashRawSync } from '../index.js'

const argon2Async = promisify(argon2)

const PASSWORD = 'test-password-for-benchmark'
const PASSWORD_BUF = Buffer.from(PASSWORD)
const SALT = Buffer.from('somesaltforsure!')

type SharedParams = {
  memoryCost: number
  timeCost: number
  parallelism: number
  outputLen: number
}

// Same numbers on every implementation. `p` is part of the tag, so it is pinned.
const CONFIGS: Array<{ name: string; params: SharedParams; rounds: number }> = [
  {
    name: 'OWASP  m=19456 KiB t=2 p=1',
    params: { memoryCost: 19456, timeCost: 2, parallelism: 1, outputLen: 32 },
    rounds: 40,
  },
  {
    name: 'RFC9106-low  m=65536 KiB t=3 p=1',
    params: { memoryCost: 65536, timeCost: 3, parallelism: 1, outputLen: 32 },
    rounds: 25,
  },
  {
    name: 'RFC9106-low  m=65536 KiB t=3 p=4',
    params: { memoryCost: 65536, timeCost: 3, parallelism: 4, outputLen: 32 },
    rounds: 25,
  },
]

type Impl = {
  name: string
  run: () => Uint8Array | Promise<Uint8Array>
}

const nodeRsOptions = (params: SharedParams) => ({
  algorithm: Algorithm.Argon2id,
  memoryCost: params.memoryCost,
  timeCost: params.timeCost,
  parallelism: params.parallelism,
  outputLen: params.outputLen,
  salt: SALT,
})

const nodeCryptoParams = (params: SharedParams) => ({
  message: PASSWORD_BUF,
  nonce: SALT,
  parallelism: params.parallelism,
  tagLength: params.outputLen,
  memory: params.memoryCost,
  passes: params.timeCost,
})

const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString('hex')

const median = (values: number[]) => {
  const sorted = [...values].sort((a, b) => a - b)
  const mid = Math.floor(sorted.length / 2)
  return sorted.length % 2 === 0 ? (sorted[mid - 1] + sorted[mid]) / 2 : sorted[mid]
}

const asBuffer = (value: Uint8Array) => Buffer.from(value)

const assertTag = (name: string, got: Uint8Array, expected: Buffer) => {
  const actual = asBuffer(got)
  if (!actual.equals(expected)) {
    throw new Error(`${name}: raw tag mismatch\n  got  ${hex(actual)}\n  want ${hex(expected)}`)
  }
}

const timeMs = async (run: Impl['run']) => {
  const start = performance.now()
  const out = await run()
  return { ms: performance.now() - start, out }
}

const interleaved = async (impls: Impl[], expected: Buffer, rounds: number) => {
  for (const impl of impls) {
    assertTag(`${impl.name} warmup`, await impl.run(), expected)
  }

  const samples = impls.map(() => [] as number[])
  for (let round = 0; round < rounds; round++) {
    // Rotate so no impl is always measured immediately after the slowest one.
    for (let offset = 0; offset < impls.length; offset++) {
      const i = (round + offset) % impls.length
      const { ms, out } = await timeMs(impls[i].run)
      assertTag(`${impls[i].name} #${round}`, out, expected)
      samples[i].push(ms)
    }
  }

  return impls.map((impl, i) => ({
    impl: impl.name,
    'median ms': Number(median(samples[i]).toFixed(2)),
    'min ms': Number(Math.min(...samples[i]).toFixed(2)),
    rounds: samples[i].length,
  }))
}

const printTable = (title: string, rows: Array<Record<string, string | number>>) => {
  console.log(`\n${title}`)
  console.table(rows)
}

for (const { name, params, rounds } of CONFIGS) {
  const expected = hashRawSync(PASSWORD, nodeRsOptions(params))

  const nativeSync: Impl[] = [
    {
      name: '@node-rs/argon2 hashRawSync',
      run: () => hashRawSync(PASSWORD, nodeRsOptions(params)),
    },
    {
      name: 'node:crypto argon2Sync',
      run: () => argon2Sync('argon2id', nodeCryptoParams(params)),
    },
  ]

  const nativeAsync: Impl[] = [
    {
      name: '@node-rs/argon2 hashRaw',
      run: () => hashRaw(PASSWORD, nodeRsOptions(params)),
    },
    {
      name: 'node-argon2 hash raw',
      run: () =>
        nodeArgon2.hash(PASSWORD, {
          type: nodeArgon2.argon2id,
          memoryCost: params.memoryCost,
          timeCost: params.timeCost,
          parallelism: params.parallelism,
          hashLength: params.outputLen,
          salt: SALT,
          version: 0x13,
          raw: true,
        }),
    },
    {
      name: 'node:crypto argon2',
      run: async () => Buffer.from(await argon2Async('argon2id', nodeCryptoParams(params))),
    },
  ]

  const jsWasm: Impl[] = [
    {
      name: 'hash-wasm argon2id binary',
      run: () =>
        wasmArgon2id({
          password: PASSWORD,
          salt: SALT,
          parallelism: params.parallelism,
          iterations: params.timeCost,
          memorySize: params.memoryCost,
          hashLength: params.outputLen,
          outputType: 'binary',
        }),
    },
    {
      name: '@noble/hashes argon2id',
      run: () =>
        nobleArgon2id(PASSWORD, SALT, {
          t: params.timeCost,
          m: params.memoryCost,
          p: params.parallelism,
          dkLen: params.outputLen,
          maxmem: 2 ** 32 - 1,
        }),
    },
  ]

  for (const impl of [...nativeSync, ...nativeAsync, ...jsWasm]) {
    assertTag(impl.name, await impl.run(), expected)
  }

  console.log(`${name}  tag=${hex(expected)}  all impls equal  interleaved x${rounds}`)

  printTable(`${name}  —  native sync raw`, await interleaved(nativeSync, expected, rounds))
  printTable(`${name}  —  native async raw`, await interleaved(nativeAsync, expected, rounds))
  printTable(`${name}  —  js/wasm raw`, await interleaved(jsWasm, expected, rounds))
}
