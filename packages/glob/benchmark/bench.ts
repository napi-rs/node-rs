import { promises as fs } from 'fs'
import path from 'path'
import { promisify } from 'util'

import b from 'benny'
import mkdirp from 'mkdirp'
import rimraf from 'rimraf'

import { globSync, glob as globAsync } from '..'
import type NpmGlobType from '../../../node_modules/@types/glob'
// @ts-expect-error
import npmGlob from '../../../node_modules/glob'

// Can't simply import "glob" because it will resolve to workspace glob due to baseUrl in tsconfig
const glob: typeof NpmGlobType = npmGlob

/** Generate a random string */
function rand() {
  return Math.random().toString(36).substring(2, 15)
}

function makeRandomFile(dir: string) {
  const file = path.join(dir, `file-${rand()}${rand()}.txt`)
  return fs.writeFile(file, '')
}

interface MakeRandomDirectoriesOption {
  /** Current direcoty full path */
  currentDir: string
  /** Depth of current directory for root of generated directory */
  crrentDepth?: number
  /** Maximum depth of directory structure */
  maxDepth: number
  /** Maximum files or directories in each generated directory */
  max: number
}

async function makeRandomDirectories({
  currentDir,
  crrentDepth = 0,
  maxDepth,
  max,
}: MakeRandomDirectoriesOption): Promise<unknown> {
  await mkdirp(currentDir)
  await Promise.all(Array.from({ length: max }).map(() => makeRandomFile(currentDir)))

  if (crrentDepth >= maxDepth) {
    return
  }

  return Promise.all(
    Array.from({ length: max }).map(() =>
      makeRandomDirectories({
        currentDir: path.join(currentDir, `dir-${rand()}${rand()}`),
        crrentDepth: crrentDepth + 1,
        maxDepth,
        max,
      }),
    ),
  )
}

async function run() {
  const benchmarkFixturesPath = path.join(__dirname, 'tmp')

  await promisify(rimraf)(benchmarkFixturesPath)
  await mkdirp(benchmarkFixturesPath)
  await makeRandomDirectories({
    currentDir: benchmarkFixturesPath,
    maxDepth: 4,
    max: 7,
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
