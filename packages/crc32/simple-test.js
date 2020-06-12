const { crc32 } = require('./index')

console.assert(crc32('hello') === '907060870')
