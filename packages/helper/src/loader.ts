import { existsSync } from 'fs'
import { platform, arch } from 'os'
import { join } from 'path'

import { platformArchTriples } from '@napi-rs/triples'

const ArchName = arch()
const PlatformName = platform()

export function loadBinding(dirname: string, filename = 'index', packageName?: string) {
  const triples = platformArchTriples[PlatformName][ArchName]
  for (const triple of triples) {
    // resolve in node_modules
    if (packageName) {
      try {
        return require(`${packageName}-${triple.platformArchABI}`)
        // eslint-disable-next-line no-empty
      } catch (e) {}
    }
    const localFilePath = join(dirname, `${filename}.${triple.platformArchABI}.node`)
    if (existsSync(localFilePath)) {
      return require(localFilePath)
    }
  }

  const errorMsg = `Can not find node binding files from ${
    packageName ? triples.map((triple) => `${packageName}-${triple.platformArchABI}`).join(', ') : ''
  } ${packageName ? 'and ' : ''}${triples
    .map((triple) => join(dirname, `${filename}.${triple.platformArchABI}.node`))
    .join(', ')}`

  throw new TypeError(errorMsg)
}
