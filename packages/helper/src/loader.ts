import { existsSync } from 'fs'
import { platform } from 'os'
import { join } from 'path'

const SupportedPlatforms = new Set<NodeJS.Platform>(['darwin', 'win32', 'linux'])

export function loadBinding(dirname: string, filename = 'index') {
  const platformName = platform()
  if (!SupportedPlatforms.has(platformName)) {
    throw new TypeError(
      `Unsupported platform: ${platformName}, only support ${[...SupportedPlatforms.values()].join(', ')}`,
    )
  }

  const bindingFilePath = join(dirname, `${filename}.${platformName}.node`)

  if (platformName === 'linux') {
    try {
      return require(bindingFilePath)
    } catch {
      return require(join(dirname, `${filename}.musl.node`))
    }
  }

  if (!existsSync(bindingFilePath)) {
    throw new TypeError(`Could not find binding file on path ${bindingFilePath}`)
  }

  return require(bindingFilePath)
}
