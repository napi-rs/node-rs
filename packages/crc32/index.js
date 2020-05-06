const { locateBinding } = require('@node-rs/helper')

const binding = require(locateBinding(__dirname, 'crc32'))

module.exports = {
  crc32: function crc32(input, crc) {
    const _input = Buffer.isBuffer(input) ? input : Buffer.from(input)
    return binding.crc32(_input, crc)
  },
  crc32c: function crc32c(input, crc) {
    const _input = Buffer.isBuffer(input) ? input : Buffer.from(input)
    return binding.crc32c(_input, crc)
  },
}
