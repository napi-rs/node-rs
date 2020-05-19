import test from 'ava'

import { loadDict, cut } from '../index'

test('should be able to load custom dict', (t) => {
  const userdict = Buffer.from('出了 10000')
  loadDict(userdict)
  const fixture = '我们中出了一个叛徒'
  t.deepEqual(cut(fixture), ['我们', '中', '出了', '一个', '叛徒'])
})
