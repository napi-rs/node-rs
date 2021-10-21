import test from 'ava'
import { h64 } from 'xxhashjs'

import { xxh64, Xxh64 } from '../index'

const FX = Buffer.from('@node-rs/xxhash vs xxhashjs')
const SEED = 0xabcdef01

test('xxh64 without seed', (t) => {
  t.is(xxh64(FX).toString(16), h64(FX, 0).toString(16))
})

test('xxh64 with seed', (t) => {
  t.is(xxh64(FX, BigInt(SEED)).toString(16), h64(FX, SEED).toString(16))
})

test('xxh64 string', (t) => {
  t.is(xxh64(FX.toString('utf8')).toString(16), h64(FX, 0).toString(16))
})

test('xxh64 Uint8Array', (t) => {
  t.is(xxh64(new Uint8Array(FX.buffer)).toString(16), h64(FX.buffer, 0).toString(16))
})

test('Xxh64 oneshot', (t) => {
  t.is(new Xxh64().update(FX).digest().toString(16), h64(FX, 0).toString(16))
})

test('Xxh64 multi steps', (t) => {
  t.is(
    new Xxh64().update(FX).update(FX).update(FX).digest().toString(16),
    h64(0).update(FX).update(FX).update(FX).digest().toString(16),
  )
})

test('Xxh64 string', (t) => {
  t.is(new Xxh64().update(FX.toString('utf8')).digest().toString(16), h64(FX, 0).toString(16))
})
