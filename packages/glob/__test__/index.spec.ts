import path from 'path'

import test from 'ava'

import { glob } from '../index'

test('gets results for the test directory', (t) => {
  const results = glob(path.join(__dirname, '../__fixtures__/*'))
  t.true(results.includes(path.join(__dirname, '../__fixtures__/test.file')))
})
