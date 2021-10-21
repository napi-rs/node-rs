import test from 'ava'
import { h32 } from 'xxhashjs'

import { xxh32, Xxh32 } from '../index'

const FX = Buffer.from('@node-rs/xxhash vs xxhashjs')
const SEED = 0xabcdef01

test('xxh32 without seed', (t) => {
  t.is(xxh32(FX), h32(FX, 0).toNumber())
})

test('xxh32 with seed', (t) => {
  t.is(xxh32(FX, SEED), h32(FX, SEED).toNumber())
})

test('xxh32 string', (t) => {
  t.is(xxh32(FX.toString('utf8')), h32(FX, 0).toNumber())
})

test('xxh32 Uint8Array', (t) => {
  t.is(xxh32(new Uint8Array(FX.buffer)), h32(FX.buffer, 0).toNumber())
})

test('Xxh32 oneshot', (t) => {
  t.is(new Xxh32().update(FX).digest(), h32(FX, 0).toNumber())
})

test('Xxh32 multi steps', (t) => {
  t.is(
    new Xxh32().update(FX).update(FX).update(FX).digest(),
    h32(0).update(FX).update(FX).update(FX).digest().toNumber(),
  )
})

test('Xxh32 string', (t) => {
  t.is(new Xxh32().update(FX.toString('utf8')).digest(), h32(FX, 0).toNumber())
})
