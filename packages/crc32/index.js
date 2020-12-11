const { loadBinding } = require('@node-rs/helper')

const binding = loadBinding(__dirname, 'crc32', '@node-rs/crc32')

module.exports = {
  crc32: function crc32(input, crc = 0) {
    const _input = Buffer.isBuffer(input) ? input : Buffer.from(input)
    return binding.crc32(_input, crc)
  },
  crc32c: function crc32c(input, crc = 0) {
    const _input = Buffer.isBuffer(input) ? input : Buffer.from(input)
    return binding.crc32c(_input, crc)
  },
}
