const { locateBinding } = require('@node-rs/helper')

const binding = require(locateBinding(__dirname, 'swc'))

module.exports = binding
