import test from 'ava'

import { xxh3 } from '../index'

const SEC_WITH_192_LENGTH = Buffer.from(
  '515293b422141cabb24c131a914d54d767738ce3f46141d91dfdfffa8b2e7ada507318f242dd112f28f213cfc1c4aba1e641e8a7f103746cc542d66668607e2ea4fce4f08640780d0bcd9f171a31f8932ae617033afd5e100c3fb6f0b5b9be611419d79c2bf2358ba1c8562ae24dd1aa2619ab30dcfaa9b8f3363b2a350e750a6aae7e307d16b1d3250f7ed6315ec127fac8643dfcb733ffe622bbc97a3097c6eabd24dee519bc7817e0e8195a426b07ad7452f6ee72465e065afe56e498a450',
  'hex',
)

test('xxh64 string', (t) => {
  t.is(xxh3.xxh64('hello world'), BigInt('15296390279056496779'))
})

test('xxh64 Buffer', (t) => {
  t.is(xxh3.xxh64(Buffer.from('hello world')), BigInt('15296390279056496779'))
})

test('xxh64 with seed', (t) => {
  t.is(xxh3.xxh64(Buffer.from('hello world'), BigInt(128)), BigInt('18072542215751182891'))
})

test('xxh64 with secret', (t) => {
  t.is(xxh3.xxh64WithSecret('hello world', SEC_WITH_192_LENGTH), BigInt('8365614992180151249'))
})

test('xxh128 string', (t) => {
  t.is(xxh3.xxh128('hello world'), BigInt('297150157938599054391163723952090887879'))
})

test('xxh128 buffer', (t) => {
  t.is(xxh3.xxh128(Buffer.from('hello world')), BigInt('297150157938599054391163723952090887879'))
})

test('xxh128 with seed', (t) => {
  t.is(xxh3.xxh128(Buffer.from('hello world'), BigInt(128)), BigInt('248039115514001876413444952452915338056'))
})

test('xxh128 with secret', (t) => {
  t.is(xxh3.xxh128WithSecret('hello world', SEC_WITH_192_LENGTH), BigInt('169165111715981571090973585540606896681'))
})

test('Xxh3 withSeed', (t) => {
  const instance = xxh3.Xxh3.withSeed()
  t.true(instance instanceof xxh3.Xxh3)
  t.is(instance.update('hello world').digest(), BigInt('15296390279056496779'))
  t.is(instance.update(Buffer.from('hello world')).digest(), BigInt('16495854690286049632'))
  instance.reset()
  t.is(instance.update('hello world').digest(), BigInt('15296390279056496779'))
})

test('Xxh3 withSecret', (t) => {
  const instance = xxh3.Xxh3.withSecret(SEC_WITH_192_LENGTH)
  t.true(instance instanceof xxh3.Xxh3)
  t.is(instance.update('hello world').digest(), BigInt('8365614992180151249'))
  t.is(instance.update(Buffer.from('hello world')).digest(), BigInt('14168446104542996972'))
  instance.reset()
  t.is(instance.update('hello world').digest(), BigInt('8365614992180151249'))
})
