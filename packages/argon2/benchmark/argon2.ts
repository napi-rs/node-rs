import { argon2, argon2Sync } from 'node:crypto'
import { promisify } from 'node:util'

import { argon2id as nobleArgon2id } from '@noble/hashes/argon2.js'
import nodeArgon2 from 'argon2'
import { argon2id as wasmArgon2id } from 'hash-wasm'
import { Bench, type Task } from 'tinybench'

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

const assertTag = (name: string, got: Uint8Array, expected: Buffer) => {
  const actual = Buffer.from(got)
  if (!actual.equals(expected)) {
    throw new Error(`${name}: raw tag mismatch\n  got  ${hex(actual)}\n  want ${hex(expected)}`)
  }
}

// A task asserts its output on every iteration — a Buffer.compare is
// sub-microsecond against an argon2 run of tens of milliseconds.
const taskFn = (impl: Impl, expected: Buffer) => async () => {
  assertTag(impl.name, await impl.run(), expected)
}

function formatLatency(ms: number): string {
  if (ms >= 1) return `${ms.toFixed(2)} ms`
  if (ms >= 0.001) return `${(ms * 1000).toFixed(2)} µs`
  return `${(ms * 1e6).toFixed(1)} ns`
}

function formatOps(opsPerSec: number): string {
  if (opsPerSec >= 1e6) return `${(opsPerSec / 1e6).toFixed(2)}M`
  if (opsPerSec >= 1e3) return `${(opsPerSec / 1e3).toFixed(2)}k`
  return opsPerSec.toFixed(2)
}

function report(bench: Bench, title: string) {
  const rows = bench.tasks
    .map((task: Task) => ({
      name: task.name,
      latency: task.result!.latency.mean,
      rme: task.result!.latency.rme,
      ops: task.result!.throughput.mean,
      samples: task.result!.latency.samplesCount,
    }))
    .sort((a, b) => a.latency - b.latency)

  // Baseline is @node-rs when the group includes it, otherwise the fastest row.
  const baselineRow = rows.find((r) => r.name.startsWith('@node-rs/')) ?? rows[0]
  const nameWidth = Math.max(...rows.map((r) => r.name.length))

  console.log(`\n${title}  (${rows[0].samples} iterations each)`)
  console.log(`  ${'implementation'.padEnd(nameWidth)}  latency       ± rme   ops/s      relative`)
  for (const row of rows) {
    const ratio = row.latency / baselineRow.latency
    const rel =
      row === baselineRow ? 'baseline' : ratio < 1 ? `${(1 / ratio).toFixed(2)}× faster` : `${ratio.toFixed(2)}× slower`
    console.log(
      `  ${row.name.padEnd(nameWidth)}  ${formatLatency(row.latency).padStart(9)}  ± ${row.rme
        .toFixed(2)
        .padStart(5)}%  ${formatOps(row.ops).padStart(8)}  ${rel}`,
    )
  }
}

for (const { name, params, rounds } of CONFIGS) {
  const expected = hashRawSync(PASSWORD, nodeRsOptions(params))

  const groups: Array<{ title: string; impls: Impl[] }> = [
    {
      title: `${name}  —  native sync raw`,
      impls: [
        {
          name: '@node-rs/argon2 hashRawSync',
          run: () => hashRawSync(PASSWORD, nodeRsOptions(params)),
        },
        {
          name: 'node:crypto argon2Sync',
          run: () => argon2Sync('argon2id', nodeCryptoParams(params)),
        },
      ],
    },
    {
      title: `${name}  —  native async raw`,
      impls: [
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
      ],
    },
    {
      title: `${name}  —  js/wasm raw`,
      impls: [
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
      ],
    },
  ]

  // Pre-flight: every implementation must produce the same tag before timing.
  for (const group of groups) {
    for (const impl of group.impls) {
      assertTag(impl.name, await impl.run(), expected)
    }
  }
  console.log(`${name}  tag=${hex(expected)}  all impls equal  ${rounds} iterations each`)

  for (const group of groups) {
    const bench = new Bench({
      name: group.title,
      // Fixed iteration count like the old harness: no time budget, no warmup
      // (the pre-flight assertion pass above already warmed each impl).
      iterations: rounds,
      time: 0,
      warmup: false,
      // A tag assertion that fails must fail the run loudly.
      throws: true,
    })
    for (const impl of group.impls) {
      bench.add(impl.name, taskFn(impl, expected))
    }
    await bench.run()
    report(bench, group.title)
  }
}
