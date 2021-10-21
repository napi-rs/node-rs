const { loadBinding } = require('@node-rs/helper')

const {
  xxh32: _xxh32,
  xxh64: _xxh64,
  Xxh32: _Xxh32,
  Xxh64: _Xxh64,
} = loadBinding(__dirname, 'xxhash', '@node-rs/xxhash')

module.exports = {
  xxh32: function xxh32(input, seed) {
    return _xxh32(Buffer.from(input), seed == null ? 0 : seed)
  },
  xxh64: function xxh64(input, seed) {
    return _xxh64(Buffer.from(input), seed == null ? BigInt(0) : seed)
  },
  Xxh32: class Xxh32 extends _Xxh32 {
    update(input) {
      return super.update(Buffer.from(input))
    }
  },
  Xxh64: class Xxh64 extends _Xxh64 {
    update(input) {
      return super.update(Buffer.from(input))
    }
  },
}
