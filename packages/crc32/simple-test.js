const { crc32 } = require('./index')

console.assert(crc32('hello') === '907060870')

console.info('crc32 simple test passed')
