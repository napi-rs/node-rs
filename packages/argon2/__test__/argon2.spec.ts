import { randomBytes } from 'crypto'

import test from 'ava'

import { Algorithm, hash, hashRaw, verify, Version } from '../index.js'

const passwordString = 'some_string123'
const passwordBuffer = Buffer.from(passwordString)

test('should allow buffer input', async (t) => {
  const hashed = await hash(passwordBuffer)
  t.true(await verify(hashed, passwordString))
})

test('should allow changing timeCost', async (t) => {
  const hashed = await hash(passwordString, {
    timeCost: 5,
  })
  t.true(await verify(hashed, passwordString))
})

test('should allow changing memoryCost', async (t) => {
  const hashed = await hash(passwordString, {
    memoryCost: 16384,
  })
  t.true(await verify(hashed, passwordString))
})

test('should allow changing parallelism', async (t) => {
  const hashed = await hash(passwordString, {
    memoryCost: 65536,
    parallelism: 2,
  })
  t.true(await verify(hashed, passwordString))
})

test('should be able to hash string', async (t) => {
  await t.notThrowsAsync(() => hash('whatever'))
  await t.notThrowsAsync(() =>
    hash('whatever', {
      secret: randomBytes(32),
    }),
  )
})

test('should be able to hashRaw string with a defined salt', async (t) => {
  await t.notThrowsAsync(() => hash('whatever'))
  await t.notThrowsAsync(() =>
    hashRaw('whatever', {
      secret: randomBytes(32),
      salt: randomBytes(32),
    }),
  )
})

test('should be able to verify hashed string', async (t) => {
  const PASSWORD = 'Argon2_is_the_best_algorithm_ever'
  t.true(await verify(await hash(PASSWORD), PASSWORD))
  t.true(
    await verify(
      await hash(PASSWORD, {
        algorithm: Algorithm.Argon2d,
      }),
      PASSWORD,
    ),
  )
  t.true(
    await verify(
      await hash(PASSWORD, {
        algorithm: Algorithm.Argon2i,
      }),
      PASSWORD,
    ),
  )
  const secret = randomBytes(32)
  t.true(
    await verify(
      await hash(PASSWORD, {
        algorithm: Algorithm.Argon2d,
        version: Version.V0x10,
        secret,
      }),
      PASSWORD,
      {
        secret,
      },
    ),
  )
})

// error
test('should return memoryCost error', async (t) => {
  const error = await t.throwsAsync(() =>
    hash(passwordString, {
      timeCost: 2,
      memoryCost: 1,
      parallelism: 1,
    }),
  )

  t.is(error?.message, 'memory cost is too small')
})

test('should return timeCost error', async (t) => {
  const error = await t.throwsAsync(() =>
    hash(passwordString, {
      timeCost: 0.6,
    }),
  )

  t.is(error?.message, 'time cost is too small')
})

test('should return parallelism error', async (t) => {
  const error = await t.throwsAsync(() =>
    hash(passwordString, {
      timeCost: 3,
      parallelism: 0,
    }),
  )

  t.is(error?.message, 'not enough threads')
})
