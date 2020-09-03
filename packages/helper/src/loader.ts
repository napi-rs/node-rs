import { existsSync } from 'fs'
import { platform } from 'os'
import { join } from 'path'

const SupportedPlatforms = new Set<NodeJS.Platform>(['darwin', 'win32', 'linux'])

export function loadBinding(dirname: string, filename = 'index', packageName?: string) {
  const platformName = platform()
  if (!SupportedPlatforms.has(platformName)) {
    throw new TypeError(
      `Unsupported platform: ${platformName}, only support for ${[...SupportedPlatforms.values()].join(', ')}`,
    )
  }

  const bindingFilePath = join(dirname, `${filename}.${platformName}.node`)
  const localMuslBindigFilePath = join(dirname, `${filename}.linux-musl.node`)
  const muslExistedInLocal = existsSync(localMuslBindigFilePath)

  if (platformName === 'linux') {
    try {
      return require(bindingFilePath)
    } catch (e1) {
      if (muslExistedInLocal) {
        try {
          return require(localMuslBindigFilePath)
        } catch (e) {
          throw new TypeError(
            `Loading linux musl addon in local path failed: ${e.message}, Loading linux addon in local path failed: ${e1.message}`,
          )
        }
      }
    }
  }

  if (!existsSync(bindingFilePath)) {
    if (!packageName) {
      throw new TypeError(`Could not find binding file on path ${bindingFilePath}`)
    }
    const platformName = platform()

    try {
      return require(`${packageName}-${platformName}`)
    } catch (e1) {
      if (platformName === 'linux') {
        try {
          return require(`${packageName}-linux-musl`)
        } catch (e) {
          throw new TypeError(`Loading linux musl addon Error: ${e.message}, Loading linux addon Error: ${e1.message}`)
        }
      } else {
        throw new TypeError('Not compatible with your platform. Error message: ' + e1.message)
      }
    }
  }

  return require(bindingFilePath)
}
