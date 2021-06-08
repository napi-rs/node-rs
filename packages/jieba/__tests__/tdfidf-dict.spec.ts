import test from 'ava'

import { loadTFIDFDict, extract } from '../index'

test('should be able to load custom dict', (t) => {
  const userdict = Buffer.from('专业 20.19')
  loadTFIDFDict(userdict)
  const fixture = '我是拖拉机学院手扶拖拉机专业的。不用多久，我就会升职加薪，当上CEO，走上人生巅峰。'
  t.snapshot(extract(fixture, 3))
})
