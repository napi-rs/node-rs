/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const __nodeFs= require('node:fs')
const __nodePath = require('node:path')
const { WASI: __nodeWASI } = require('node:wasi')
const { Worker } = require('node:worker_threads')

const {
  instantiateNapiModuleSync: __emnapiInstantiateNapiModuleSync,
  getDefaultContext: __emnapiGetDefaultContext,
} = require('@napi-rs/wasm-runtime')

const __wasi = new __nodeWASI({
  version: 'preview1',
  env: process.env,
  preopens: {
    '/': '/'
  }
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 1024,
  maximum: 10240,
  shared: true,
})

let __wasmFilePath = __nodePath.join(__dirname, 'jsonwebtoken.wasm32-wasi.wasm')

if (!__nodeFs.existsSync(__wasmFilePath)) {
  try {
    __wasmFilePath = __nodePath.resolve('@node-rs/jsonwebtoken-wasm32-wasi')
  } catch {
    throw new Error('Cannot find jsonwebtoken.wasm32-wasi.wasm file, and @node-rs/jsonwebtoken-wasm32-wasi package is not installed.')
  }
}

const { instance: __napiInstance, module: __wasiModule, napiModule: __napiModule } = __emnapiInstantiateNapiModuleSync(__nodeFs.readFileSync(__wasmFilePath), {
  context: __emnapiContext,
  asyncWorkPoolSize: (function() {
    const threadsSizeFromEnv = Number(process.env.NAPI_RS_ASYNC_WORK_POOL_SIZE ?? process.env.UV_THREADPOOL_SIZE)
    // NaN > 0 is false
    if (threadsSizeFromEnv > 0) {
      return threadsSizeFromEnv
    } else {
      return 4
    }
  })(),
  wasi: __wasi,
  onCreateWorker() {
    return new Worker(__nodePath.join(__dirname, 'wasi-worker.mjs'), {
      env: process.env,
      execArgv: ['--experimental-wasi-unstable-preview1'],
    })
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance)
  }
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__Algorithm_0']?.()
  __napiInstance.exports['__napi_register__decode_header_1']?.()
  __napiInstance.exports['__napi_register__Header_struct_2']?.()
  __napiInstance.exports['__napi_register__SignTask_impl_3']?.()
  __napiInstance.exports['__napi_register__sign_4']?.()
  __napiInstance.exports['__napi_register__sign_sync_5']?.()
  __napiInstance.exports['__napi_register__Validation_struct_6']?.()
  __napiInstance.exports['__napi_register__VerifyTask_impl_7']?.()
  __napiInstance.exports['__napi_register__verify_8']?.()
  __napiInstance.exports['__napi_register__verify_sync_9']?.()
}
module.exports.Algorithm = __napiModule.exports.Algorithm
module.exports.decodeHeader = __napiModule.exports.decodeHeader
module.exports.sign = __napiModule.exports.sign
module.exports.signSync = __napiModule.exports.signSync
module.exports.verify = __napiModule.exports.verify
module.exports.verifySync = __napiModule.exports.verifySync
