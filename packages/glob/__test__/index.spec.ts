import path from 'path'

import test from 'ava'

import { globSync, glob } from '../index'

test('globSync', (t) => {
  const results = globSync(path.join(__dirname, '../__fixtures__/*'))
  t.true(results.includes(path.join(__dirname, '../__fixtures__/test.file')))
})

test('glob', async (t) => {
  const results = await glob(path.join(__dirname, '../__fixtures__/*'))
  t.true(results.includes(path.join(__dirname, '../__fixtures__/test.file')))
})
