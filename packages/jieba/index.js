const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let isMusl = false
let loadError = null

switch (platform) {
  case 'android':
    if (arch !== 'arm64') {
      throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'jieba.android-arm64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./jieba.android-arm64.node')
      } else {
        nativeBinding = require('@node-rs/jieba-android-arm64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'jieba.win32-x64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./jieba.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@node-rs/jieba-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(join(__dirname, 'jieba.win32-ia32-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./jieba.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@node-rs/jieba-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'jieba.win32-arm64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./jieba.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@node-rs/jieba-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'jieba.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./jieba.darwin-x64.node')
          } else {
            nativeBinding = require('@node-rs/jieba-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'jieba.darwin-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./jieba.darwin-arm64.node')
          } else {
            nativeBinding = require('@node-rs/jieba-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'jieba.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./jieba.freebsd-x64.node')
      } else {
        nativeBinding = require('@node-rs/jieba-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
        if (isMusl) {
          localFileExisted = existsSync(join(__dirname, 'jieba.linux-x64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./jieba.linux-x64-musl.node')
            } else {
              nativeBinding = require('@node-rs/jieba-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'jieba.linux-x64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./jieba.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@node-rs/jieba-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
        if (isMusl) {
          localFileExisted = existsSync(join(__dirname, 'jieba.linux-arm64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./jieba.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@node-rs/jieba-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'jieba.linux-arm64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./jieba.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@node-rs/jieba-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'jieba.linux-arm-gnueabihf.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./jieba.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('@node-rs/jieba-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { load, loadDict, cut, cutAll, cutForSearch, tag, extract, loadTFIDFDict } = nativeBinding

module.exports.load = load
module.exports.loadDict = loadDict
module.exports.cut = cut
module.exports.cutAll = cutAll
module.exports.cutForSearch = cutForSearch
module.exports.tag = tag
module.exports.extract = extract
module.exports.loadTFIDFDict = loadTFIDFDict
