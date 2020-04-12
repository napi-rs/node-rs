const fs = require('fs')
const path = require('path')
const { Suite } = require('benchmark')

const { parse } = require('../lib')

const filepath = path.join(__dirname, 'json-examples', 'apache-builds.json')
const apacheJSONString = fs.readFileSync(filepath, 'utf8')
const ab = Buffer.from('"{}"').buffer

const suite = new Suite()

suite.add('SIMD + NAPI', () => {
  parse(ab)
}).add('Native JSON parse', () => {
  JSON.parse("{}")
})
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
.run()
