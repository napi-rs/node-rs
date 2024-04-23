import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
  createOnMessage as __wasmCreateOnMessageForFsProxy,
} from '@napi-rs/wasm-runtime'

import __wasmUrl from './bcrypt.wasm32-wasi.wasm?url'

const __wasi = new __WASI({
  version: 'preview1',
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
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
  },
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__HashTask_impl_0']?.()
  __napiInstance.exports['__napi_register__SaltTask_impl_1']?.()
  __napiInstance.exports['__napi_register__VerifyTask_impl_2']?.()
  __napiInstance.exports['__napi_register__DEFAULT_COST_3']?.()
  __napiInstance.exports['__napi_register__gen_salt_sync_4']?.()
  __napiInstance.exports['__napi_register__gen_salt_js_5']?.()
  __napiInstance.exports['__napi_register__hash_sync_6']?.()
  __napiInstance.exports['__napi_register__hash_7']?.()
  __napiInstance.exports['__napi_register__verify_sync_8']?.()
  __napiInstance.exports['__napi_register__verify_9']?.()
}
export const DEFAULT_COST = __napiModule.exports.DEFAULT_COST
export const genSalt = __napiModule.exports.genSalt
export const genSaltSync = __napiModule.exports.genSaltSync
export const hash = __napiModule.exports.hash
export const hashSync = __napiModule.exports.hashSync
export const verify = __napiModule.exports.verify
export const verifySync = __napiModule.exports.verifySync
