const { DEFAULT_COST, genSaltSync, genSalt, hashSync, hash, verifySync, verify } = require('./binding')

module.exports.DEFAULT_COST = DEFAULT_COST
module.exports.genSaltSync = genSaltSync
module.exports.genSalt = genSalt
module.exports.hashSync = hashSync
module.exports.hash = hash
module.exports.verifySync = verifySync
module.exports.verify = verify
module.exports.compareSync = verifySync
module.exports.compare = verify
