import test from 'ava'

import { load } from '../index'

test('should be able to load', (t) => {
  const fn = () => load()
  t.notThrows(fn)
})
