import { argon2Sync, randomBytes } from 'node:crypto'

import test from 'ava'

import nodeArgon2 from 'argon2'

import { Algorithm, hash, hashRaw, hashRawSync, hashSync, verify, Version } from '../index.js'

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

test('should be able to hash string with a defined salt', async (t) => {
  await t.notThrowsAsync(() =>
    hash('whatever', {
      salt: randomBytes(32),
    }),
  )
  await t.notThrowsAsync(() =>
    hash('whatever', {
      secret: randomBytes(32),
      salt: randomBytes(32),
    }),
  )

  const salt = randomBytes(32)
  t.is(
    await hash('whatever', {
      salt,
    }),
    await hash('whatever', {
      salt,
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

  t.is(error?.message, 'Memory cost is too small')
})

test('should return timeCost error', async (t) => {
  const error = await t.throwsAsync(() =>
    hash(passwordString, {
      timeCost: 0.6,
    }),
  )

  t.is(error?.message, 'Time cost is too small')
})

test('should return parallelism error', async (t) => {
  const error = await t.throwsAsync(() =>
    hash(passwordString, {
      timeCost: 3,
      parallelism: 0,
    }),
  )

  t.is(error?.message, 'Too few lanes')
})

test('should match node-argon2 on the same params and salt', async (t) => {
  const password = '1:1-compare-password'
  const salt = Buffer.from('somesaltforsure!')
  const options = {
    algorithm: Algorithm.Argon2id,
    memoryCost: 4096,
    timeCost: 1,
    parallelism: 1,
    outputLen: 32,
    salt,
  }

  const encoded = await hash(password, options)
  const raw = await hashRaw(password, options)
  const theirs = await nodeArgon2.hash(password, {
    type: nodeArgon2.argon2id,
    memoryCost: options.memoryCost,
    timeCost: options.timeCost,
    parallelism: options.parallelism,
    hashLength: options.outputLen,
    salt,
    version: 0x13,
  })
  const theirsRaw = await nodeArgon2.hash(password, {
    type: nodeArgon2.argon2id,
    memoryCost: options.memoryCost,
    timeCost: options.timeCost,
    parallelism: options.parallelism,
    hashLength: options.outputLen,
    salt,
    version: 0x13,
    raw: true,
  })

  // node-argon2 emits `m=,p=,t=`; the C reference and argon2-rust emit
  // `m=,t=,p=`. The tag bytes are the 1:1 comparison.
  t.deepEqual(raw, theirsRaw)
  t.true(await verify(theirs, password))
  t.true(await nodeArgon2.verify(encoded, password))
  t.is(hashSync(password, options), encoded)
  t.deepEqual(hashRawSync(password, options), raw)
  t.deepEqual(
    raw,
    Buffer.from(
      argon2Sync('argon2id', {
        message: Buffer.from(password),
        nonce: salt,
        parallelism: options.parallelism,
        tagLength: options.outputLen,
        memory: options.memoryCost,
        passes: options.timeCost,
      }),
    ),
  )
})

test('should verify node-argon2 hashes that carry associatedData', async (t) => {
  const password = '1:1-compare-password'
  const salt = Buffer.from('somesaltforsure!')
  const associatedData = Buffer.from('phc-associated-data')
  const secret = randomBytes(16)
  const shared = {
    type: nodeArgon2.argon2id,
    memoryCost: 4096,
    timeCost: 1,
    parallelism: 1,
    hashLength: 32,
    salt,
    version: 0x13,
    associatedData,
  }

  const withAd = await nodeArgon2.hash(password, shared)
  t.true(withAd.includes('data='))
  t.true(await verify(withAd, password))
  t.false(await verify(withAd, 'wrong-password'))

  const withAdAndSecret = await nodeArgon2.hash(password, { ...shared, secret })
  t.true(await verify(withAdAndSecret, password, { secret }))
  t.false(await verify(withAdAndSecret, password))
})
