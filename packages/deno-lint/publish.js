const { execSync } = require('child_process')
const fs = require('fs')
const path = require('path')

const platforms = require('../../scripts/platforms')

const { version } = require('./package.json')
const updatePackageJson = require('./update-package')

updatePackageJson(path.join(__dirname, 'package.json'), {
  optionalDependencies: platforms.reduce((acc, cur) => {
    acc[`@node-rs/deno-lint-${cur}`] = `^${version}`
    return acc
  }, {}),
})

for (const name of platforms) {
  const pkgDir = path.join(__dirname, 'npm', name)
  const filename = `deno-lint.${name}.node`
  const bindingFile = fs.readFileSync(path.join(__dirname, `bindings-${name}`, filename))
  fs.writeFileSync(path.join(pkgDir, filename), bindingFile)
  execSync('npm publish', {
    cwd: pkgDir,
    env: process.env,
    stdio: 'inherit',
  })
}
