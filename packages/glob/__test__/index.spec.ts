import test from 'ava'

import { globPattern } from '../index'

test('sync function from native code', (t) => {
  t.plan(1)
  globPattern('.', (str) => {
    t.is(str, ['Hello'])
  })
})
