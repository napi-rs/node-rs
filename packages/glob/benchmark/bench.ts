import path from 'path'
import { promisify } from 'util'

import b from 'benny'
import fastGlob from 'fast-glob'
import fs from 'graceful-fs'
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
  const extensions = ['txt', 'log', 'code', 'py']
  const extension = extensions[Math.floor(Math.random() * extensions.length)]
  const file = path.join(dir, `file-${rand()}${rand()}.${extension}`)
  return promisify(fs.writeFile)(file, '')
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
}: MakeRandomDirectoriesOption): Promise<void> {
  await mkdirp(currentDir)
  await Promise.all(Array.from({ length: max }).map(() => makeRandomFile(currentDir)))

  if (crrentDepth >= maxDepth) {
    return
  }

  await Promise.all(
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
    maxDepth: 5,
    max: 20,
  })

  for await (const pattern of [
    '**/*.txt',
    '**/*a*.txt',
    '*a*/*b*/*c.txt',
    '**/*[0-9]*.code',
    '*.py',
    '**/dir-*/dir-**/dir-**/**/*.code',
  ]) {
    const pathAndPattern = path.join(benchmarkFixturesPath, pattern)

    await b.suite(
      `${pattern} sync`,

      b.add('"glob"', () => {
        glob.sync(pathAndPattern)
      }),

      b.add('"fast-glob', () => {
        fastGlob.sync(pathAndPattern)
      }),

      b.add('"@napi-rs/glob"', () => {
        globSync(pathAndPattern)
      }),

      b.cycle(),
      b.complete(),
    )

    await b.suite(
      `${pattern} async`,

      b.add('"glob"', async () => {
        await promisify(glob)(pathAndPattern)
      }),

      b.add('"fast-glob', async () => {
        await fastGlob(pathAndPattern)
      }),

      b.add('"@napi-rs/glob"', async () => {
        await globAsync(pathAndPattern)
      }),

      b.cycle(),
      b.complete(),
    )
  }
}

run().catch(console.error)
