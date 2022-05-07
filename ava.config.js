module.exports = {
  extensions: ['ts'],
  workerThreads: false,
  require: ['@swc-node/register'],
  files: ['packages/**/*.spec.ts'],
  timeout: '3m',
  environmentVariables: {
    TS_NODE_PROJECT: './tsconfig.test.json',
  },
}
