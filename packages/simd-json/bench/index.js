const fs = require('fs')
const path = require('path')
const { Suite } = require('benchmark')

const { parse } = require('../lib')

const filepath = path.join(__dirname, 'json-examples', 'apache-builds.json')
const apacheJSONString = `
{
  "assignedLabels" : [
    {
      
    }
  ],
  "mode" : "EXCLUSIVE",
  "nodeDescription" : "the master Jenkins node",
  "nodeName" : "",
  "numExecutors" : 0
}
`
const jsonBuffer = Buffer.from(
  apacheJSONString
)

const suite = new Suite()

suite.add('SIMD + NAPI', () => {
  parse(jsonBuffer)
}).add('Native JSON parse', () => {
  JSON.parse(apacheJSONString)
})
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
.run()
