import openbsd from '@cwasm/openbsd-bcrypt'
import openwall from '@cwasm/openwall-bcrypt'
import bcryptjs from 'bcryptjs'
import nodeBcrypt from 'bcrypt'
import { Bench, type Task } from 'tinybench'

import { compareSync, genSaltSync, hashSync } from '../index.js'

const PASSWORD = 'node-rust-password'
const COST = 10
// Fixed salt: hashing measures the key-expansion loop only, so the suites stay
// comparable when salt generation differs in cost between implementations.
const SALT = '$2b$10$KBCwKxOzLha2MUDgW0PjXe'
const HASH = hashSync(PASSWORD, { salt: SALT })

interface Implementation {
  name: string
  // Uniform signatures so every task gets the same arguments.
  hash: (password: string) => string
  verify: (password: string, hash: string) => boolean
  genSalt: () => string
}

const implementations: Implementation[] = [
  {
    name: '@node-rs/bcrypt',
    hash: (password) => hashSync(password, { salt: SALT }),
    verify: (password, hash) => compareSync(password, hash),
    genSalt: () => genSaltSync({ cost: COST }),
  },
  {
    name: 'node bcrypt (C++)',
    hash: (password) => nodeBcrypt.hashSync(password, SALT),
    verify: (password, hash) => nodeBcrypt.compareSync(password, hash),
    genSalt: () => nodeBcrypt.genSaltSync(COST),
  },
  {
    name: 'bcryptjs',
    hash: (password) => bcryptjs.hashSync(password, SALT),
    verify: (password, hash) => bcryptjs.compareSync(password, hash),
    genSalt: () => bcryptjs.genSaltSync(COST),
  },
  {
    name: 'wasm OpenBSD',
    // The cwasm wrappers generate the salt inside hashSync and take the cost —
    // the ~1µs gensalt is included for these two rows only.
    hash: (password) => openbsd.hashSync(password, COST),
    verify: (password, hash) => openbsd.compareSync(password, hash),
    genSalt: () => openbsd.genSaltSync(COST),
  },
  {
    name: 'wasm Openwall',
    hash: (password) => openwall.hashSync(password, COST),
    verify: (password, hash) => openwall.compareSync(password, hash),
    genSalt: () => openwall.genSaltSync(COST),
  },
]

function formatLatency(ms: number): string {
  if (ms >= 1) return `${ms.toFixed(2)} ms`
  if (ms >= 0.001) return `${(ms * 1000).toFixed(2)} µs`
  return `${(ms * 1e6).toFixed(1)} ns`
}

function formatOps(opsPerSec: number): string {
  if (opsPerSec >= 1e6) return `${(opsPerSec / 1e6).toFixed(2)}M`
  if (opsPerSec >= 1e3) return `${(opsPerSec / 1e3).toFixed(2)}k`
  return opsPerSec.toFixed(1)
}

function report(bench: Bench, title: string) {
  const rows = bench.tasks
    .map((task: Task) => {
      const r = task.result!
      return {
        name: task.name,
        latency: r.latency.mean,
        rme: r.latency.rme,
        ops: r.throughput.mean,
        samples: r.latency.samplesCount,
      }
    })
    .sort((a, b) => a.latency - b.latency)

  const baseline = rows.find((r) => r.name === '@node-rs/bcrypt')?.latency ?? rows[0].latency
  const nameWidth = Math.max(...rows.map((r) => r.name.length))

  console.log(`\n${title}  (password ${PASSWORD.length}B, ${rows[0].samples} samples)`)
  console.log(`  ${'implementation'.padEnd(nameWidth)}  latency       ± rme   ops/s      relative`)
  for (const row of rows) {
    const ratio = row.latency / baseline
    const rel =
      row.name === '@node-rs/bcrypt'
        ? 'baseline'
        : ratio < 1
          ? `${(1 / ratio).toFixed(2)}× faster`
          : `${ratio.toFixed(2)}× slower`
    console.log(
      `  ${row.name.padEnd(nameWidth)}  ${formatLatency(row.latency).padStart(9)}  ± ${row.rme
        .toFixed(2)
        .padStart(5)}%  ${formatOps(row.ops).padStart(8)}  ${rel}`,
    )
  }
}

const hashBench = new Bench({ name: 'hash' })
const verifyBench = new Bench({ name: 'verify' })
const genSaltBench = new Bench({ name: 'genSalt' })

for (const impl of implementations) {
  hashBench.add(impl.name, () => impl.hash(PASSWORD))
  verifyBench.add(impl.name, () => impl.verify(PASSWORD, HASH))
  genSaltBench.add(impl.name, () => impl.genSalt())
}

for (const [bench, title] of [
  [hashBench, `hashSync(password, fixed-salt-cost-${COST})`],
  [verifyBench, `verifySync(password, hash)`],
  [genSaltBench, `genSaltSync(cost ${COST})`],
] as const) {
  await bench.run()
  report(bench, title)
}
