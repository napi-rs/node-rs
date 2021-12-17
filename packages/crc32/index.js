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
    localFileExisted = existsSync(join(__dirname, 'crc32.android-arm64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./crc32.android-arm64.node')
      } else {
        nativeBinding = require('@node-rs/crc32-android-arm64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'crc32.win32-x64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./crc32.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@node-rs/crc32-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(join(__dirname, 'crc32.win32-ia32-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./crc32.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@node-rs/crc32-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'crc32.win32-arm64-msvc.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./crc32.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@node-rs/crc32-win32-arm64-msvc')
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
        localFileExisted = existsSync(join(__dirname, 'crc32.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./crc32.darwin-x64.node')
          } else {
            nativeBinding = require('@node-rs/crc32-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'crc32.darwin-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./crc32.darwin-arm64.node')
          } else {
            nativeBinding = require('@node-rs/crc32-darwin-arm64')
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
    localFileExisted = existsSync(join(__dirname, 'crc32.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./crc32.freebsd-x64.node')
      } else {
        nativeBinding = require('@node-rs/crc32-freebsd-x64')
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
          localFileExisted = existsSync(join(__dirname, 'crc32.linux-x64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./crc32.linux-x64-musl.node')
            } else {
              nativeBinding = require('@node-rs/crc32-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'crc32.linux-x64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./crc32.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@node-rs/crc32-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
        if (isMusl) {
          localFileExisted = existsSync(join(__dirname, 'crc32.linux-arm64-musl.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./crc32.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@node-rs/crc32-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(join(__dirname, 'crc32.linux-arm64-gnu.node'))
          try {
            if (localFileExisted) {
              nativeBinding = require('./crc32.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@node-rs/crc32-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'crc32.linux-arm-gnueabihf.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./crc32.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('@node-rs/crc32-linux-arm-gnueabihf')
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

const { crc32c, crc32 } = nativeBinding

module.exports.crc32c = crc32c
module.exports.crc32 = crc32
