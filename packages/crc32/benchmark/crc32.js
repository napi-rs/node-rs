const { Suite } = require('benchmark')
const Sse4Crc32 = require('sse4_crc32')
const { crc32: crc32Node } = require('crc')
const chalk = require('chalk')

const { crc32c, crc32 } = require('../crc32.node')

const TEST_BUFFER = Buffer.from(`Lorem ipsum dolor sit amet, consectetur
adipiscing elit. Morbi mollis cursus metus vel tristique. Proin congue massa
massa, a malesuada dolor ullamcorper a. Nulla eget leo vel orci venenatis
placerat. Donec semper condimentum justo, vel sollicitudin dolor consequat id.
Nunc sed aliquet felis, eget congue nisi. Mauris eu justo suscipit, elementum
turpis ut, molestie tellus. Mauris ornare rutrum fringilla. Nulla dignissim
luctus pretium. Nullam nec eros hendrerit sapien pellentesque sollicitudin.
Integer eget ligula dui. Mauris nec cursus nibh. Nunc interdum elementum leo, eu
sagittis eros sodales nec. Duis dictum nulla sed tincidunt malesuada. Quisque in
vulputate sapien. Sed sit amet tellus a est porta rhoncus sed eu metus. Mauris
non pulvinar nisl, volutpat luctus enim. Suspendisse est nisi, sagittis at risus
quis, ultricies rhoncus sem. Donec ullamcorper purus eget sapien facilisis, eu
eleifend felis viverra. Suspendisse elit neque, semper aliquet neque sed,
egestas tempus leo. Duis condimentum turpis duis.`)

const initialCrc32 = crc32Node(TEST_BUFFER)
const initialCrc32c = Sse4Crc32.calculate(TEST_BUFFER)

console.assert(crc32(TEST_BUFFER), initialCrc32)
console.assert(crc32c(TEST_BUFFER), initialCrc32c)

const suite = new Suite('crc32c without initial crc')

suite
  .add('SIMD + NAPI', () => {
    crc32c(TEST_BUFFER)
  })
  .add('sse4_crc32', () => {
    Sse4Crc32.calculate(TEST_BUFFER)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

const suite2 = new Suite('crc32c with initial crc')

suite2
  .add('SIMD + NAPI', () => {
    crc32c(TEST_BUFFER, initialCrc32c)
  })
  .add('sse4_crc32', () => {
    Sse4Crc32.calculate(TEST_BUFFER, initialCrc32c)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

const suite3 = new Suite('crc32 without initial crc')

suite3
  .add('SIMD + NAPI', () => {
    crc32(TEST_BUFFER)
  })
  .add('Node crc', () => {
    crc32Node(TEST_BUFFER)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()

const suite4 = new Suite('crc32 with initial crc')

suite4
  .add('SIMD + NAPI', () => {
    crc32(TEST_BUFFER, initialCrc32)
  })
  .add('Node crc32', () => {
    crc32Node(TEST_BUFFER, initialCrc32)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()
