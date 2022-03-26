const USE_TS_NODE = (function () {
  try {
    require('@swc-node/core')
    return false
  } catch (e) {
    return true
  }
})()

module.exports = {
  extensions: ['ts'],
  workerThreads: true,
  require: [
    USE_TS_NODE ? 'ts-node/register/transpile-only' : '@swc-node/register',
    '@node-rs/argon2',
    '@node-rs/bcrypt',
    '@node-rs/crc32',
    '@node-rs/deno-lint',
    '@node-rs/jieba',
    '@node-rs/xxhash',
  ],
  files: ['packages/**/*.spec.ts'],
  timeout: '3m',
  environmentVariables: {
    TS_NODE_PROJECT: './tsconfig.test.json',
  },
}
