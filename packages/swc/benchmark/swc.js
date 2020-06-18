const fs = require('fs')

const { transform } = require('@swc/core')
const { Suite } = require('benchmark')
const chalk = require('chalk')
const { startService } = require('esbuild')
const { range } = require('lodash')

const { transform: napiTransform } = require('../index')

const suite = new Suite('Uglify three.js benchmark')

const THREE_JS_SOURCE_CODE = fs.readFileSync(require.resolve('three'))
const THREE_JS_SOURCE_CODE_STRING = THREE_JS_SOURCE_CODE.toString('utf8')
const parallel = 6

async function run() {
  const service = await startService()

  suite
    .add('@swc/core', {
      fn: (deferred) => {
        Promise.all(
          range(0, parallel).map(() =>
            transform(THREE_JS_SOURCE_CODE_STRING, {
              minify: true,
            }),
          ),
        ).then(() => {
          deferred.resolve()
        })
      },
      defer: true,
    })
    .add('@node-rs/swc', {
      fn: (deferred) => {
        Promise.all(range(0, parallel).map(() => napiTransform(THREE_JS_SOURCE_CODE))).then(() => {
          deferred.resolve()
        })
      },
      defer: true,
    })
    .add('esbuild', {
      fn: (deferred) => {
        Promise.all(
          range(0, parallel).map(() =>
            service.transform(THREE_JS_SOURCE_CODE_STRING, {
              minify: true,
              sourcemap: false,
            }),
          ),
        ).then(() => {
          deferred.resolve()
        })
      },
      defer: true,
    })
    .on('cycle', function (event) {
      event.target.hz = event.target.hz * parallel
      console.info(String(event.target))
    })
    .on('complete', function () {
      console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
      service.stop()
    })
    .run()
}

run().catch((e) => {
  console.error(e)
})
