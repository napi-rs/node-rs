const { cpus } = require('os')

const nodeArgon2 = require('argon2')
const { Suite } = require('benchmark')
const chalk = require('chalk')

const { hash, verify, Algorithm } = require('../index')

const PASSWORD = '$v=19$m=4096,t=3,p=1$fyLYvmzgpBjDTP6QSypj3g$pb1Q3Urv1amxuFft0rGwKfEuZPhURRDV7TJqcBnwlGo'
const CORES = cpus().length

const suite = new Suite('Hash with all cores')

suite
  .add(
    '@node-rs/argon',
    async (deferred) => {
      await hash(PASSWORD, {
        algorithm: Algorithm.Argon2id,
        parallelism: CORES,
      })
      deferred.resolve()
    },
    { defer: true },
  )
  .add(
    'node-argon',
    async (deferred) => {
      await nodeArgon2.hash(PASSWORD, { type: nodeArgon2.argon2id, parallelism: CORES })
      deferred.resolve()
    },
    {
      defer: true,
    },
  )
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()
