import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
  createOnMessage as __wasmCreateOnMessageForFsProxy,
} from '@napi-rs/wasm-runtime'

import __wasmUrl from './jsonwebtoken.wasm32-wasi.wasm?url'

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
export const Algorithm = __napiModule.exports.Algorithm
export const decodeHeader = __napiModule.exports.decodeHeader
export const sign = __napiModule.exports.sign
export const signSync = __napiModule.exports.signSync
export const verify = __napiModule.exports.verify
export const verifySync = __napiModule.exports.verifySync
