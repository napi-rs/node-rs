const { loadBinding } = require('@node-rs/helper')

const {
  xxh32: _xxh32,
  xxh64: _xxh64,
  Xxh32: _Xxh32,
  Xxh64: _Xxh64,
  xxh3,
} = loadBinding(__dirname, 'xxhash', '@node-rs/xxhash')

class Xxh3 {
  update(data) {
    return xxh3.update.call(this, Buffer.from(data))
  }
}

Xxh3.withSecret = function withSecret(secret) {
  const instance = new Xxh3()
  xxh3.createXxh3WithSecret(instance, Buffer.from(secret))
  return instance
}

Xxh3.withSeed = function withSeed(seed = BigInt(0)) {
  const instance = new Xxh3()
  xxh3.createXxh3WithSeed(instance, seed)
  return instance
}

Xxh3.prototype.digest = xxh3.digest
Xxh3.prototype.reset = xxh3.reset

module.exports = {
  xxh32: function xxh32(input, seed) {
    return _xxh32(Buffer.isBuffer(input) ? input : Buffer.from(input), seed == null ? 0 : seed)
  },
  xxh64: function xxh64(input, seed) {
    return _xxh64(Buffer.isBuffer(input) ? input : Buffer.from(input), seed == null ? BigInt(0) : seed)
  },
  Xxh32: class Xxh32 extends _Xxh32 {
    update(input) {
      return super.update(Buffer.isBuffer(input) ? input : Buffer.from(input))
    }
  },
  Xxh64: class Xxh64 extends _Xxh64 {
    update(input) {
      return super.update(Buffer.isBuffer(input) ? input : Buffer.from(input))
    }
  },
  xxh3: {
    xxh64: function xxh64(input, seed) {
      return xxh3.xxh64(Buffer.isBuffer(input) ? input : Buffer.from(input), seed == null ? BigInt(0) : seed)
    },
    xxh64WithSecret(input, secret) {
      return xxh3.xxh64WithSecret(Buffer.isBuffer(input) ? input : Buffer.from(input), Buffer.from(secret))
    },
    xxh128: function xxh128(input, seed) {
      return xxh3.xxh128(Buffer.isBuffer(input) ? input : Buffer.from(input), seed == null ? BigInt(0) : seed)
    },
    xxh128WithSecret(input, secret) {
      return xxh3.xxh128WithSecret(Buffer.isBuffer(input) ? input : Buffer.from(input), Buffer.from(secret))
    },
    Xxh3,
  },
}
