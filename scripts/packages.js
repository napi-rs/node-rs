const { readdirSync, existsSync, statSync } = require('fs')
const { join } = require('path')

const packagesDir = join(__dirname, '..', 'packages')

module.exports = readdirSync(packagesDir)
  .filter((dir) => statSync(join(packagesDir, dir)).isDirectory())
  .filter((dir) => existsSync(join(packagesDir, dir, 'Cargo.toml')))
