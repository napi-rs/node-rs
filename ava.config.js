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
  require: [USE_TS_NODE ? 'ts-node/register/transpile-only' : '@swc-node/register'],
  files: ['packages/**/*.spec.ts'],
  timeout: '3m',
  environmentVariables: {
    TS_NODE_PROJECT: './tsconfig.test.json',
  },
}
