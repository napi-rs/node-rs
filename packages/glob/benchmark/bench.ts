import { execFileSync } from 'child_process'
import path from 'path'

import b from 'benny'
import glob from 'glob'

import { globSync } from '../index'

async function run() {
  execFileSync(path.join(__dirname, 'make-benchmark-fixtures.sh'))

  await b.suite(
    'Glob "**/*.txt" sync',

    b.add('npm "glob" package', () => {
      glob.globSync('/tmp/**/*.txt')
    }),

    b.add('"@napi-rs/glob"', () => {
      globSync('/tmp/**/*.txt')
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch(console.error)
