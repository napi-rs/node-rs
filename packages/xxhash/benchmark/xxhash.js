const { readFileSync } = require('fs')
const { join } = require('path')

const { Suite } = require('benchmark')
const chalk = require('chalk')
const { h32: h32js, h64: h64js } = require('xxhashjs')

const { xxh32, xxh64, Xxh32, Xxh64 } = require('../index')

const FX = readFileSync(join(__dirname, '..', '..', '..', 'yarn.lock'))

new Suite('xxh32')
  .add('@node-rs/xxhash h32', () => {
    xxh32(FX)
  })
  .add('xxhashjs h32', () => {
    h32js(FX, 0).toNumber()
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

new Suite('xxh32 multi steps')
  .add('@node-rs/xxhash h32', () => {
    new Xxh32().update(FX).digest()
  })
  .add('xxhashjs h32', () => {
    h32js().update(FX).digest().toNumber()
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

new Suite('xxh64')
  .add('@node-rs/xxhash 64', () => {
    xxh64(FX).toString(16)
  })
  .add('xxhashjs h64', () => {
    h64js(FX, 0).toString(16)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

new Suite('xxh64 multi steps')
  .add('@node-rs/xxhash 64', () => {
    new Xxh64().update(FX).digest().toString(16)
  })
  .add('xxhashjs h64', () => {
    h64js(0).update(FX).digest().toString(16)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()
