const avaConfig = {
  extensions: ['ts'],
  workerThreads: false,
  cache: false,
  require: ['@swc-node/register'],
  files: ['packages/**/*.spec.ts'],
  timeout: '3m',
  environmentVariables: {
    TS_NODE_PROJECT: './tsconfig.test.json',
  },
}

if (process.env.NAPI_RS_FORCE_WASI) {
  avaConfig.files.push(`!packages/jsonwebtoken/**/*.spec.ts`)
}

export default avaConfig
