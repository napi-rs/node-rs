const { loadBinding } = require('@node-rs/helper')

const binding = loadBinding(__dirname, 'bcrypt', '@node-rs/bcrypt')

const DEFAULT_COST = 12

function verify(password, hash) {
  password = Buffer.isBuffer(password) ? password : Buffer.from(password)
  hash = Buffer.isBuffer(hash) ? hash : Buffer.from(hash)
  return binding.verify(password, hash)
}

function verifySync(password, hash) {
  password = Buffer.isBuffer(password) ? password : Buffer.from(password)
  hash = Buffer.isBuffer(hash) ? hash : Buffer.from(hash)
  return binding.verifySync(password, hash)
}

module.exports = {
  DEFAULT_COST: DEFAULT_COST,

  genSaltSync: function genSaltSync(round = 10, version = '2b') {
    return binding.genSaltSync(round, version)
  },

  genSalt: function genSalt(round = 10, version = '2b') {
    return binding.genSalt(round, version)
  },

  hashSync: function hashSync(password, round = DEFAULT_COST) {
    const input = Buffer.isBuffer(password) ? password : Buffer.from(password)
    return binding.hashSync(input, round)
  },

  hash: function hash(password, round = DEFAULT_COST) {
    const input = Buffer.isBuffer(password) ? password : Buffer.from(password)
    return binding.hash(input, round)
  },

  verifySync,
  verify,
  compareSync: verifySync,
  compare: verify,
}
