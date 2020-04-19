const { Suite } = require('benchmark')
const Sse4Crc32 = require('sse4_crc32')

const { calculate } = require('../index.node')

const suite = new Suite()

const fixture = 'hello world'

const fx = Buffer.from(fixture)

suite.add('SIMD + NAPI', () => {
  calculate(fx)
}).add('sse4_crc32', () => {
  Sse4Crc32.calculate(fx)
})
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
.run()
