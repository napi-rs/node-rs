import { existsSync } from 'fs'
import { platform, arch } from 'os'
import { join } from 'path'

import { platformArchTriples } from '@napi-rs/triples'

/**
 * @param requireFn NodeJS require function, you must pass your `require` in because `yarn pnp` need it, see https://github.com/napi-rs/node-rs/issues/263 for detail
 * @param dirname The path to try to load native binding while developing in local
 * @param filename napi.name field in your package.json
 * @param packageName your package name, `@node-rs/helper` will try to load native package by using this name concat with `triple.platform-arch-abi`
 */
export function loadBinding(requireFn: typeof require, dirname: string, filename = 'index', packageName?: string) {
  const ArchName = arch()
  const PlatformName = platform()

  const triples = platformArchTriples[PlatformName][ArchName]
  for (const triple of triples) {
    // resolve in node_modules
    if (packageName) {
      try {
        return requireFn(`${packageName}-${triple.platformArchABI}`)
        // eslint-disable-next-line no-empty
      } catch (e) {}
    }
    const localFilePath = join(dirname, `${filename}.${triple.platformArchABI}.node`)
    if (existsSync(localFilePath)) {
      return requireFn(localFilePath)
    }
  }

  const errorMsg = `Can not find node binding files from ${
    packageName ? triples.map((triple) => `${packageName}-${triple.platformArchABI}`).join(', ') : ''
  } ${packageName ? 'and ' : ''}${triples
    .map((triple) => join(dirname, `${filename}.${triple.platformArchABI}.node`))
    .join(', ')}`

  throw new TypeError(errorMsg)
}
