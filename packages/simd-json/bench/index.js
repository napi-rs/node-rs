const fs = require('fs')
const path = require('path')
const { Suite } = require('benchmark')
const { range } = require('lodash')

const { parse } = require('../lib')

const suite = new Suite()

const filepath = path.join(__dirname, 'json-examples', 'apache-builds.json')
const apacheJSON = fs.readFileSync(filepath)
const apacheJSONString = fs.readFileSync(filepath, 'utf8')

const RUN_TIMES = range(0, 10000)

for (const _ of RUN_TIMES) {
  parse(apacheJSON)
}

// suite.add('JSON Parse native', function() {
//   JSON.parse(apacheJSONString)
// })
// .add('SIMD JSON Parse', function() {
//   parse(apacheJSON)
// })
// .on('cycle', function(event) {
//   console.log(String(event.target))
// })
// .on('complete', function() {
//   console.log('Fastest is ' + this.filter('fastest').map('name'))
// })
// .run()
