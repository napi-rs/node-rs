const { execSync } = require('child_process')
const { join } = require('path')
const { existsSync } = require('fs')
const chalk = require('chalk')
const putasset = require('putasset')
const { Octokit } = require('@octokit/rest')

const supporttedPlatforms = require('./platforms')

const headCommit = execSync('git log -1 --pretty=%B', {
  encoding: 'utf8',
})

;(async () => {
  const [owner, repo] = process.env.GITHUB_REPOSITORY.split('/')
  const octokit = new Octokit({
    auth: process.env.GITHUB_TOKEN,
  })
  const packages = headCommit
    .split('\n')
    .map((line) => line.trim())
    .filter((line, index) => line.length && index)
    .map((line) => line.substr(2))
    .map(parseTag)
    .filter((pkgInfo) => pkgInfo.isNativePackage)
  for (const package of packages) {
    await octokit.repos.createRelease({
      owner,
      repo,
      tag_name: package.tag,
    })
    await Promise.all(
      supporttedPlatforms.map(async (platform) => {
        const binary = join(package.pkgDir, `${package.name}.${platform}.node`)
        const downloadUrl = await putasset(process.env.GITHUB_TOKEN, {
          owner,
          repo,
          tag: package.tag,
          filename: binary,
        })
        console.info(`${chalk.green(binary)} upload success`)
        console.info(`Download url: ${chalk.blueBright(downloadUrl)}`)
      }),
    )
  }
})()

/**
 * @param {string} tag
 */
function parseTag(tag) {
  const [, packageWithVersion] = tag.split('/')
  const [name, version] = packageWithVersion.split('@')
  const pkgDir = join(__dirname, '..', 'packages', name)
  const tomlFilePath = join(pkgDir, 'Cargo.toml')
  const isNativePackage = existsSync(tomlFilePath)

  return {
    name,
    version,
    pkgDir,
    tag,
    isNativePackage,
  }
}
