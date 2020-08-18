const { platform } = require('os')

const { loadBinding } = require('@node-rs/helper')

let binding

try {
  binding = loadBinding(__dirname, 'deno-lint')
  // eslint-disable-next-line no-empty
} catch (e) {}

if (!binding) {
  const platformName = platform()
  try {
    binding = require(`@node-rs/deno-lint-${platformName}`)
  } catch (e) {
    if (platformName === 'linux') {
      try {
        binding = require('@node-rs/deno-lint-linux-musl')
      } catch (e) {
        throw new TypeError('Error loading native addon: ' + e.message)
      }
    } else {
      throw new TypeError('Not compatible with your platform. Error message: ' + e.message)
    }
  }
}

module.exports = {
  binding,
  lint: function lint(path, sourcecode, allRules = false) {
    const source = Buffer.isBuffer(sourcecode) ? sourcecode : Buffer.from(sourcecode)
    return binding.lint(path, source, allRules)
  },
}
