const { execSync } = require('child_process')
const { existsSync } = require('fs')
const { platform } = require('os')
const { join } = require('path')

const packages = require('./packages')
const supporttedPlatforms = require('./platforms')

const MOVE_ALL = process.env.MOVE_TARGET === 'all'

const platforms = MOVE_ALL ? supporttedPlatforms : [platform()]

/**
 * @param {string[]} _platforms platforms
 * @param {boolean | undefined} moveAll
 * @returns {void}
 */
function moveFile(_platforms, moveAll = false) {
  for (const package of packages) {
    for (const platformName of _platforms) {
      const optionalPath = moveAll ? `bindings-${platformName}` : ''
      const artifactPath = join(process.cwd(), 'artifacts', optionalPath, package, `${package}.${platformName}.node`)
      if (existsSync(artifactPath)) {
        execSync(`mv ${artifactPath} ./packages/${package}/`, {
          stdio: 'inherit',
        })
      } else {
        throw new TypeError(`${artifactPath} not existed`)
      }
    }
  }
}

moveFile(platforms, MOVE_ALL)
