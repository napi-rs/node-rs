const { loadBinding } = require('@node-rs/helper')

const binding = loadBinding(__dirname, 'swc')

module.exports = {
  ...binding,
  transformSync: function transformSync(input) {
    const source = Buffer.isBuffer(input) ? input : Buffer.from(input)
    return binding.transformSync(source)
  },
}
