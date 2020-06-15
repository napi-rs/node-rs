const { platform } = require('os')

const { loadBinding } = require('@node-rs/helper')

let binding

try {
  binding = loadBinding(__dirname, 'deno-lint')
} catch (e) {
  try {
    binding = require(`@node-rs/deno-lint-${platform()}`)
  } catch (e) {
    throw new TypeError('Not compatible with your platform. Error message: ' + e.message)
  }
}

module.exports = {
  lint: function lint(filename, sourcecode) {
    const source = Buffer.isBuffer(sourcecode) ? sourcecode : Buffer.from(sourcecode)
    return binding.lint(filename, source)
  },
}
