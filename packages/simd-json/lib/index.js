const isProduction = process.env.NODE_ENV === 'production'

module.exports = require(isProduction ? '../index.release.node' : '../index.node')
