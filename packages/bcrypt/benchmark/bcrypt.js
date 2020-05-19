const { cpus } = require('os')

const { hashSync, hash, compare } = require('bcrypt')
const { Suite } = require('benchmark')
const chalk = require('chalk')
const { range } = require('lodash')

const { hash: napiHash, hashSync: napiHashSync, verify } = require('../index')

const hashRounds = [10, 12, 14]
const parallel = cpus().length

const password = 'node-rust-password'

function runAsync(round) {
  const asyncHashSuite = new Suite(`Async hash round ${round}`)
  return new Promise((resolve) => {
    asyncHashSuite
      .add('@node-rs/bcrypt', {
        defer: true,
        fn: (deferred) => {
          Promise.all(range(parallel).map(() => napiHash(password, round))).then(() => {
            deferred.resolve()
          })
        },
      })
      .add('node bcrypt', {
        defer: true,
        fn: (deferred) => {
          Promise.all(range(parallel).map(() => hash(password, round))).then(() => {
            deferred.resolve()
          })
        },
      })
      .on('cycle', function (event) {
        event.target.hz = event.target.hz * parallel
        console.info(String(event.target))
      })
      .on('complete', function () {
        console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
        resolve()
      })
      .run({ async: true })
  })
}

hashRounds
  .reduce(async (acc, cur) => {
    await acc
    return runAsync(cur)
  }, Promise.resolve())
  .then(
    () =>
      new Promise((resolve) => {
        const suite = new Suite('Async verify')
        const hash = napiHashSync(password)
        suite
          .add({
            name: '@node-rs/bcrypt',
            defer: true,
            fn: (deferred) => {
              Promise.all(range(parallel).map(() => verify(password, hash))).then(() => {
                deferred.resolve()
              })
            },
          })
          .add({
            name: 'node bcrypt',
            defer: true,
            fn: (deferred) => {
              Promise.all(range(parallel).map(() => compare(password, hash))).then(() => {
                deferred.resolve()
              })
            },
          })
          .on('cycle', function (event) {
            event.target.hz = event.target.hz * parallel
            console.info(String(event.target))
          })
          .on('complete', function () {
            resolve()
            console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
          })
          .run()
      }),
  )
  .then(() => {
    for (const round of hashRounds) {
      const syncHashSuite = new Suite(`Hash round ${round}`)
      syncHashSuite
        .add('@node-rs/bcrypt', () => {
          napiHashSync(password, round)
        })
        .add('node bcrypt', () => {
          hashSync(password, round)
        })
        .on('cycle', function (event) {
          console.info(String(event.target))
        })
        .on('complete', function () {
          console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
        })
        .run()
    }
  })
