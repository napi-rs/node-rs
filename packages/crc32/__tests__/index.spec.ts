import test from 'ava'
import { crc32 as nodeCrc32 } from 'crc'

import { crc32c, crc32 } from '../index'

const fx = Buffer.from('crc32c - test')

test('crc32c result should be equal with sse4_crc32', (t) => {
  t.snapshot(crc32c(fx))
})

test('crc32c result should be equal with sse4_crc32 when calculate with initial crc', (t) => {
  const initialCrc = crc32c(fx)
  t.snapshot(crc32c(fx, initialCrc))
})

test('crc32 result should be equal with crc32 node', (t) => {
  t.is(crc32(fx), nodeCrc32(fx))
})

test('crc32 result should be equal with crc32 node when caclulate with initial crc', (t) => {
  const initialCrc = crc32c(fx)
  t.is(crc32(fx, initialCrc), nodeCrc32(fx, initialCrc))
})
