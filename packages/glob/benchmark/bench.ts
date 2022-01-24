import { execFileSync } from 'child_process'
import path from 'path'
import { promisify } from 'util'

import b from 'benny'

import { globSync, glob as globAsync } from '..'
import type NpmGlobType from '../../../node_modules/@types/glob'
// @ts-expect-error
import npmGlob from '../../../node_modules/glob'

// Can't simply import "glob" because it will resolve to workspace glob due to baseUrl in tsconfig
const glob: typeof NpmGlobType = npmGlob

async function run() {
  const benchmarkFixturesPath = path.join(__dirname, 'tmp')

  execFileSync(path.join(__dirname, 'make-benchmark-fixtures.sh'), {
    env: {
      TMPDIR: benchmarkFixturesPath,
    },
    stdio: 'pipe',
  })

  await b.suite(
    'Glob "**/*.txt" sync',

    b.add('"glob"', () => {
      glob.sync(path.join(benchmarkFixturesPath, '**/*.txt'))
    }),

    b.add('"@napi-rs/glob"', () => {
      globSync(path.join(benchmarkFixturesPath, '**/*.txt'))
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'Glob "**/*.txt" async',

    b.add('"glob"', async () => {
      await promisify(glob)(path.join(benchmarkFixturesPath, '**/*.txt'))
    }),

    b.add('"@napi-rs/glob"', async () => {
      await globAsync(path.join(benchmarkFixturesPath, '**/*.txt'))
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch(console.error)
