import * as nodeCrypto from 'node:crypto'
import { randomBytes } from 'node:crypto'

import test from 'ava'

import { Algorithm, hash, hashRaw, hashRawSync, hashSync, verify, Version } from '../index.js'

const argon2Sync = typeof nodeCrypto.argon2Sync === 'function' ? nodeCrypto.argon2Sync.bind(nodeCrypto) : undefined
const interop = argon2Sync ? test : test.skip

const b64 = (value: Uint8Array) => Buffer.from(value).toString('base64').replace(/=+$/, '')

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

interop('should match node:crypto argon2 on the same params and salt', async (t) => {
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
  const builtin = Buffer.from(
    argon2Sync!('argon2id', {
      message: Buffer.from(password),
      nonce: salt,
      parallelism: options.parallelism,
      tagLength: options.outputLen,
      memory: options.memoryCost,
      passes: options.timeCost,
    }),
  )

  t.deepEqual(raw, builtin)
  t.true(await verify(encoded, password))
  t.is(hashSync(password, options), encoded)
  t.deepEqual(hashRawSync(password, options), raw)
})

interop('should verify PHC strings that carry associatedData', async (t) => {
  const password = '1:1-compare-password'
  const salt = Buffer.from('somesaltforsure!')
  const associatedData = Buffer.from('phc-associated-data')
  const secret = randomBytes(16)
  const shared = {
    message: Buffer.from(password),
    nonce: salt,
    parallelism: 1,
    tagLength: 32,
    memory: 4096,
    passes: 1,
    associatedData,
  }

  const withAd = Buffer.from(argon2Sync!('argon2id', shared))
  const phc = `$argon2id$v=19$m=4096,t=1,p=1,data=${b64(associatedData)}$${b64(salt)}$${b64(withAd)}`
  t.true(await verify(phc, password))
  t.false(await verify(phc, 'wrong-password'))

  const withAdAndSecret = Buffer.from(argon2Sync!('argon2id', { ...shared, secret }))
  const phcKeyed = `$argon2id$v=19$m=4096,t=1,p=1,data=${b64(associatedData)}$${b64(salt)}$${b64(withAdAndSecret)}`
  t.true(await verify(phcKeyed, password, { secret }))
  t.false(await verify(phcKeyed, password))
})
