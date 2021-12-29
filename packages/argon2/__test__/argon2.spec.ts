import { randomBytes } from 'crypto'

import test from 'ava'

import { Algorithm, hash, verify, Version } from '../index.js'

test('should be able to hash string', async (t) => {
  await t.notThrowsAsync(() => hash('whatever'))
  await t.notThrowsAsync(() =>
    hash('whatever', {
      secret: randomBytes(32),
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
