import { existsSync, readdirSync } from 'fs'
import { platform, arch } from 'os'
import { join } from 'path'

import { platformArchTriples } from '@napi-rs/triples'

const ArchName = arch()
const PlatformName = platform()

export function loadBinding(dirname: string, filename = 'index', packageName?: string) {
  const triples = platformArchTriples[PlatformName][ArchName]
  let additionalErrorMsg = ''
  for (const triple of triples) {
    // resolve in node_modules
    if (packageName) {
      try {
        return require(require.resolve(`${packageName}-${triple.platformArchABI}`, { paths: [dirname] }))
      } catch (e: any) {
        if (e?.code !== 'MODULE_NOT_FOUND') {
          try {
            const pkgPath = require.resolve(`${packageName}-${triple.platformArchABI}`, {
              paths: [dirname],
            })
            additionalErrorMsg += `file: ${pkgPath} existed but error occurred while require it: ${e.message ?? e} \n`
            // eslint-disable-next-line no-empty
          } catch {}
        }
      }
    }
    const localFilePath = join(dirname, `${filename}.${triple.platformArchABI}.node`)
    if (existsSync(localFilePath)) {
      try {
        return require(localFilePath)
      } catch (e: any) {
        additionalErrorMsg += `file: ${localFilePath} existed but error occurred while require it: ${e.message ?? e} \n`
      }
    }
  }

  let packageList = ''

  if (packageName) {
    try {
      // @swc/core => core
      // awesome-package => awesome-package
      const packageNameWithoutNamespace = packageName.split('/').pop()!
      packageList = readdirSync(join(require.resolve(packageName, { paths: [dirname] }), '..', '..'))
        .filter((d) => d !== packageNameWithoutNamespace && d.startsWith(packageNameWithoutNamespace))
        .join(', ')
      // eslint-disable-next-line no-empty
    } catch {}
  }

  const errorMsg = `Can not load bindings${additionalErrorMsg ? ', ' + additionalErrorMsg : '\n'}${
    packageList ? 'Installed packages: [' + packageList + ']' : ''
  }`

  throw new Error(errorMsg)
}
