import path from 'path'

import test from 'ava'

import { globPattern } from '../index'

test('gets results for the test directory', (t) => {
  t.plan(1)
  globPattern(path.join(__dirname, '__fixtures__/*'), (strs) => {
    t.is(strs[0], path.join(__dirname, '__fixtures__/test.file'))
  })
})
