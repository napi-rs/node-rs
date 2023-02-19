const { cpus } = require('os')
const { hrtime } = require('process')

const { hashSync, hash, compare, genSaltSync } = require('bcrypt')
const { hashSync: hashSyncJs, hash: hashJs, compare: compareJs, genSaltSync: genSaltSyncJs } = require('bcryptjs')
const { Suite } = require('benchmark')
const chalk = require('chalk')
const { range } = require('lodash')
const { from, timer, lastValueFrom, Subject } = require('rxjs')
const { mergeMap, takeUntil } = require('rxjs/operators')

const { hash: napiHash, hashSync: napiHashSync, verify, genSaltSync: napiGenSaltSync } = require('../index')

const parallel = cpus().length - 1

const password = 'node-rust-password'

const CPU_LENGTH = cpus().length

const DEFAULT_TOTAL_ITERATIONS = 10000
const DEFAULT_MAX_DURATION = 20000

function bench(name, options = {}) {
  const suites = []
  return {
    add(suiteName, suiteFn) {
      suites.push({
        name: suiteName,
        fn: suiteFn,
      })
      return this
    },
    run: async () => {
      let fastest = {
        perf: -1,
        name: '',
      }
      for (const { suiteName, fn: suiteFn } of suites) {
        try {
          await suiteFn()
        } catch (e) {
          console.error(`Warming up ${suiteName} failed`)
          throw e
        }
      }
      for (const { name: suiteName, fn: suiteFn } of suites) {
        const iterations = options.iterations ?? DEFAULT_TOTAL_ITERATIONS
        const parallel = options.parallel ?? CPU_LENGTH
        const maxDuration = options.maxDuration ?? DEFAULT_MAX_DURATION
        const start = hrtime.bigint()
        let totalIterations = 0
        let finishedIterations = 0
        const finish$ = new Subject()
        await lastValueFrom(
          from({ length: iterations }).pipe(
            mergeMap(async () => {
              totalIterations++
              await suiteFn()
              finishedIterations++
              if (finishedIterations === totalIterations) {
                finish$.next()
                finish$.complete()
              }
            }, parallel),
            takeUntil(timer(maxDuration)),
          ),
        )
        if (finishedIterations !== totalIterations) {
          await lastValueFrom(finish$)
        }
        const duration = Number(hrtime.bigint() - start)
        const currentPerf = totalIterations / duration
        if (currentPerf > fastest.perf) {
          fastest = {
            perf: currentPerf,
            name: suiteName,
          }
        }
        console.info(`${suiteName} ${Math.round(currentPerf * 1e9)} ops/s`)
      }
      console.info(`In ${name} suite, fastest is ${fastest.name}`)
    },
  }
}

bench(`Hash round 12`)
  .add('@node-rs/bcrypt', () => napiHash(password, 12))
  .add('node bcrypt', () => hash(password, 12))
  .run()
  .catch((err) => {
    console.error(err)
    process.exit(1)
  })

// function runAsync(round = 12) {
//   const asyncHashSuite = new Suite(`Async hash round ${round}`)
//   return new Promise((resolve) => {
//     asyncHashSuite
//       .add('@node-rs/bcrypt', {
//         defer: true,
//         fn: (deferred) => {
//           Promise.all(range(parallel).map(() => napiHash(password, round))).then(() => {
//             deferred.resolve()
//           })
//         },
//       })
//       .add('node bcrypt', {
//         defer: true,
//         fn: (deferred) => {
//           Promise.all(range(parallel).map(() => hash(password, round))).then(() => {
//             deferred.resolve()
//           })
//         },
//       })
//       .add('bcryptjs', {
//         defer: true,
//         fn: (deferred) => {
//           Promise.all(range(parallel).map(() => hashJs(password, round))).then(() => {
//             deferred.resolve()
//           })
//         },
//       })
//       .on('cycle', function (event) {
//         event.target.hz = event.target.hz * parallel
//         console.info(String(event.target))
//       })
//       .on('complete', function () {
//         console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
//         resolve()
//       })
//       .run({ async: true })
//   })
// }

// runAsync()
//   .then(
//     () =>
//       new Promise((resolve) => {
//         const suite = new Suite('Async verify')
//         const hash = napiHashSync(password)
//         suite
//           .add({
//             name: '@node-rs/bcrypt',
//             defer: true,
//             fn: (deferred) => {
//               Promise.all(range(parallel).map(() => verify(password, hash))).then(() => {
//                 deferred.resolve()
//               })
//             },
//           })
//           .add({
//             name: 'node bcrypt',
//             defer: true,
//             fn: (deferred) => {
//               Promise.all(range(parallel).map(() => compare(password, hash))).then(() => {
//                 deferred.resolve()
//               })
//             },
//           })
//           .add({
//             name: 'bcryptjs',
//             defer: true,
//             fn: (deferred) => {
//               Promise.all(range(parallel).map(() => compareJs(password, hash))).then(() => {
//                 deferred.resolve()
//               })
//             },
//           })
//           .on('cycle', function (event) {
//             event.target.hz = event.target.hz * parallel
//             console.info(String(event.target))
//           })
//           .on('complete', function () {
//             resolve()
//             console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
//           })
//           .run()
//       }),
//   )
//   .then(() => {
//     return new Promise((resolve) => {
//       const syncHashSuite = new Suite(`Hash round 12`)
//       syncHashSuite
//         .add('@node-rs/bcrypt', () => {
//           napiHashSync(password, 12)
//         })
//         .add('node bcrypt', () => {
//           hashSync(password, 12)
//         })
//         .add('bcryptjs', () => {
//           hashSyncJs(password, 12)
//         })
//         .on('cycle', function (event) {
//           console.info(String(event.target))
//         })
//         .on('complete', function () {
//           console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
//           resolve()
//         })
//         .run()
//     })
//   })
//   .then(() => {
//     return new Promise((resolve) => {
//       new Suite('genSaltSync')
//         .add('@node-rs/bcrypt', () => {
//           napiGenSaltSync(10, '2b')
//         })
//         .add('node bcrypt', () => {
//           genSaltSync(10, 'b')
//         })
//         .add('bcryptjs', () => {
//           genSaltSyncJs(10)
//         })
//         .on('cycle', function (event) {
//           console.info(String(event.target))
//         })
//         .on('complete', function () {
//           console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
//           resolve()
//         })
//         .run()
//     })
//   })
