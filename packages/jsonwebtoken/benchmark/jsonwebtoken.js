const { cpus } = require('os')

const { Suite } = require('benchmark')
const chalk = require('chalk')
const nodeJsonwebtoken = require('jsonwebtoken')
const { range } = require('lodash')

const { sign, verify, verifySync, signSync } = require('../index')

const getUtcTimestamp = () => Math.floor(new Date().getTime() / 1000)
const oneDayInSeconds = 86400

function nodeJwtSignAsync(jwtPayload, signKey) {
  return new Promise((resolve, reject) => {
    nodeJsonwebtoken.sign(jwtPayload, signKey, (err, token) => {
      if (err) {
        return reject(err)
      }
      resolve(token)
    })
  })
}
function nodeJwtVerifyAsync(jwt, verifyKey) {
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

const numOfCores = cpus().length

function runSignAsync() {
  return new Promise((resolve) => {
    const suite = new Suite('Async sign')

    return suite
      .add('@node-rs/jsonwebtoken', {
        defer: true,
        fn: (deferred) => {
          const asyncJobs = range(numOfCores).map(() => sign(jwtClaims, secretKey))
          Promise.all(asyncJobs).then(() => deferred.resolve())
        },
      })
      .add('node-jsonwebtoken', {
        defer: true,
        fn: (deferred) => {
          const asyncJobs = range(numOfCores).map(() => nodeJwtSignAsync(jwtClaims, secretKey))
          Promise.all(asyncJobs).then(() => deferred.resolve())
        },
      })
      .on('cycle', function (event) {
        console.info(String(event.target))
      })
      .on('complete', function () {
        console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
        resolve()
      })
      .run()
  })
}

function runVerifyAsync() {
  return new Promise((resolve) => {
    const suite = new Suite('Async verify')

    return suite
      .add('@node-rs/jsonwebtoken', {
        defer: true,
        fn: (deferred) => {
          const asyncJobs = range(numOfCores).map(() => verify(token, secretKey))
          Promise.all(asyncJobs).then(() => deferred.resolve())
        },
      })
      .add('node-jsonwebtoken', {
        defer: true,
        fn: (deferred) => {
          const asyncJobs = range(numOfCores).map(() => nodeJwtVerifyAsync(token, secretKey))
          Promise.all(asyncJobs).then(() => deferred.resolve())
        },
      })
      .on('cycle', function (event) {
        console.info(String(event.target))
      })
      .on('complete', function () {
        console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
        resolve()
      })
      .run()
  })
}

function runSignSync() {
  return new Promise((resolve) => {
    const suite = new Suite('Sync sign')

    return suite
      .add('@node-rs/jsonwebtoken', () => {
        signSync(jwtClaims, secretKey)
      })
      .add('node-jsonwebtoken', () => {
        nodeJwtSignSync(jwtClaims, secretKey)
      })
      .on('cycle', function (event) {
        console.info(String(event.target))
      })
      .on('complete', function () {
        console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
        resolve()
      })
      .run()
  })
}

function runVerifySync() {
  return new Promise((resolve) => {
    const suite = new Suite('Sync verify')

    return suite
      .add('@node-rs/jsonwebtoken', () => {
        verifySync(token, secretKey)
      })
      .add('node-jsonwebtoken', () => {
        nodeJwtVerifySync(token, secretKey)
      })
      .on('cycle', function (event) {
        console.info(String(event.target))
      })
      .on('complete', function () {
        console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
        resolve()
      })
      .run()
  })
}

// Run suites in series
;(async () => {
  const suiteRunners = [runSignAsync, runVerifyAsync, runSignSync, runVerifySync]
  for (const run of suiteRunners) {
    await run()
  }
})()
