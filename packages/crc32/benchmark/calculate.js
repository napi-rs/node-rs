const { Suite } = require('benchmark')
const Sse4Crc32 = require('sse4_crc32')
const chalk = require('chalk')

const { crc32c } = require('../index.node')

const suite = new Suite('Without initial crc')

const fixture = 'hello world'

const fx = Buffer.from(fixture)

suite
  .add('SIMD + NAPI', () => {
    crc32c(fx)
  })
  .add('sse4_crc32', () => {
    Sse4Crc32.calculate(fx)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

const suite2 = new Suite('With initial crc')
const fx2 = Buffer.from('amazing napi + rust')

suite2
  .add('SIMD + NAPI', () => {
    crc32c(fx2, 3381945770)
  })
  .add('sse4_crc32', () => {
    Sse4Crc32.calculate(fx2, 3381945770)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()
