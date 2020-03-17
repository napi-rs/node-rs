const fs = require('fs')
const { promisify } = require('util')

const readFileAsync = promisify(fs.readFile)

const { decode: nativeDecode } = require('../index.node')

module.exports.decode = async function decode(filepathOrBuffer) {
  if (Buffer.isBuffer(filepathOrBuffer)) {
    return nativeDecode(filepathOrBuffer)
  }
  const pngData = await readFileAsync(filepathOrBuffer)
  return nativeDecode(pngData)
}
