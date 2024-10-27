import { Bench } from 'tinybench'
import chalk from 'chalk'
import nodeJsonwebtoken, { type PrivateKey, type Secret, type PublicKey, type GetPublicKeyOrSecret } from 'jsonwebtoken'

import { sign, verify, verifySync, signSync } from '../index.js'

const getUtcTimestamp = () => Math.floor(new Date().getTime() / 1000)
const oneDayInSeconds = 86400

function nodeJwtSignAsync(jwtPayload: Parameters<typeof nodeJsonwebtoken.sign>[0], signKey: Secret | PrivateKey) {
  return new Promise((resolve, reject) => {
    nodeJsonwebtoken.sign(jwtPayload, signKey, (err, token) => {
      if (err) {
        return reject(err)
      }
      resolve(token)
    })
  })
}
function nodeJwtVerifyAsync(jwt: string, verifyKey: Secret | PublicKey | GetPublicKeyOrSecret) {
  return new Promise((resolve, reject) => {
    nodeJsonwebtoken.verify(jwt, verifyKey, (err, token) => {
      if (err) {
        return reject(err)
      }
      resolve(token)
    })
  })
}
const nodeJwtSignSync = nodeJsonwebtoken.sign
const nodeJwtVerifySync = nodeJsonwebtoken.verify

const secretKey = 'qwertyuiopasdfghjklzxcvbnm123456'
const jwtClaims = {
  data: {
    id: 'f81d4fae-7dec-11d0-a765-00a0c91e6bf6',
    pr: 33,
    isM: true,
    set: ['KL', 'TV', 'JI'],
    nest: { id: 'poly' },
  },
  exp: getUtcTimestamp() + oneDayInSeconds,
}
const token = nodeJwtSignSync(jwtClaims, secretKey)

const suite = new Bench()

await suite
  .add('@node-rs/jsonwebtoken', async () => {
    await sign(jwtClaims, secretKey)
  })
  .add('@node-rs/jsonwebtoken sync', () => {
    signSync(jwtClaims, secretKey)
  })
  .add('jsonwebtoken', async () => {
    await nodeJwtSignAsync(jwtClaims, secretKey)
  })
  .add('jsonwebtoken sync', () => {
    nodeJwtSignSync(jwtClaims, secretKey)
  })
  .warmup()

await suite.run()
console.info(chalk.green('Sign token'))
console.table(suite.table())

const verifySuite = new Bench()

await verifySuite
  .add('@node-rs/jsonwebtoken', async () => {
    await verify(token, secretKey)
  })
  .add('@node-rs/jsonwebtoken sync', () => {
    verifySync(token, secretKey)
  })
  .add('jsonwebtoken', async () => {
    await nodeJwtVerifyAsync(token, secretKey)
  })
  .add('jsonwebtoken sync', () => {
    nodeJwtVerifySync(token, secretKey)
  })
  .warmup()

await verifySuite.run()
console.info(chalk.green('Verify token'))
console.table(verifySuite.table())
