const { loadBinding } = require('@node-rs/helper')

const binding = loadBinding(__dirname, 'deno-lint', '@node-rs/deno-lint')

module.exports = {
  ...binding,
  lint: function lint(path, sourcecode, allRules = false) {
    const source = Buffer.isBuffer(sourcecode) ? sourcecode : Buffer.from(sourcecode)
    return binding.lint(path, source, allRules)
  },
}
