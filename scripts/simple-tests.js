const packages = require('./packages')

for (const pkg of packages) {
  require(`../packages/${pkg}/simple-test.js`)
}
