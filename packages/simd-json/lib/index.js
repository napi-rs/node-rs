const isProduction = process.env.NODE_ENV === 'production'

const { parse: nativeParse } = require(isProduction ? '../index.release.node' : '../index.node')

module.exports.parse = nativeParse
