import { platform } from 'os'
import { join } from 'path'
import { existsSync } from 'fs'

const SupportedPlatforms = new Set<NodeJS.Platform>(['darwin', 'win32', 'linux'])

export function locateBinding(dirname: string) {
  const platformName = platform()
  if (!SupportedPlatforms.has(platformName)) {
    throw new TypeError(
      `Unsupported platform: ${platformName}, only support ${[...SupportedPlatforms.values()].join(', ')}`,
    )
  }

  const bindingFilePath = join(dirname, `index.${platformName}.node`)

  if (!existsSync(bindingFilePath)) {
    throw new TypeError(`Could not find binding file on path ${bindingFilePath}`)
  }

  return bindingFilePath
}
