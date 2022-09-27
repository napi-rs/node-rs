const { cpus } = require('os')

const openbsd = require('@cwasm/openbsd-bcrypt')
const openwall = require('@cwasm/openwall-bcrypt')
const { hashSync, hash, compare, genSaltSync } = require('bcrypt')
const { hashSync: hashSyncJs, hash: hashJs, compare: compareJs, genSaltSync: genSaltSyncJs } = require('bcryptjs')
const { Suite } = require('benchmark')
const chalk = require('chalk')
const { range } = require('lodash')

const { hash: napiHash, hashSync: napiHashSync, verify, genSaltSync: napiGenSaltSync } = require('../index')

const parallel = cpus().length

const password = 'node-rust-password'

function runAsync(round = 12) {
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
      .add('bcryptjs', {
        defer: true,
        fn: (deferred) => {
          Promise.all(range(parallel).map(() => hashJs(password, round))).then(() => {
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

runAsync()
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
          .add({
            name: 'bcryptjs',
            defer: true,
            fn: (deferred) => {
              Promise.all(range(parallel).map(() => compareJs(password, hash))).then(() => {
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
    return new Promise((resolve) => {
      const syncHashSuite = new Suite(`Hash round 12`)
      syncHashSuite
        .add('@node-rs/bcrypt', () => {
          napiHashSync(password, 12)
        })
        .add('node bcrypt', () => {
          hashSync(password, 12)
        })
        .add('bcryptjs', () => {
          hashSyncJs(password, 12)
        })
        .add('wasm OpenBSD', () => {
          openbsd.hashSync(password, 12)
        })
        .add('wasm Openwall', () => {
          openwall.hashSync(password, 12)
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
  })
  .then(() => {
    return new Promise((resolve) => {
      new Suite('genSaltSync')
        .add('@node-rs/bcrypt', () => {
          napiGenSaltSync(10, '2b')
        })
        .add('node bcrypt', () => {
          genSaltSync(10, 'b')
        })
        .add('bcryptjs', () => {
          genSaltSyncJs(10)
        })
        .add('wasm OpenBSD', () => {
          openbsd.genSaltSync(10)
        })
        .add('wasm Openwall', () => {
          openwall.genSaltSync(10)
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
  })
