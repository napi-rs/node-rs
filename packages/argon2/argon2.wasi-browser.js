import { instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync } from '@emnapi/core'
import { getDefaultContext as __emnapiGetDefaultContext } from '@emnapi/runtime'
import { WASI as __WASI } from '@tybys/wasm-util'
import { Volume as __Volume, createFsFromVolume as __createFsFromVolume } from 'memfs-browser'

import __wasmUrl from './argon2.wasm32-wasi.wasm?url'

const __fs = __createFsFromVolume(
  __Volume.fromJSON({
    '/': null,
  }),
)

const __wasi = new __WASI({
  version: 'preview1',
  fs: __fs,
})

const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 1024,
  maximum: 10240,
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
    return new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
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
  },
})

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports['__napi_register__Algorithm_0']?.()
  __napiInstance.exports['__napi_register__Version_1']?.()
  __napiInstance.exports['__napi_register__Options_struct_2']?.()
  __napiInstance.exports['__napi_register__HashTask_impl_3']?.()
  __napiInstance.exports['__napi_register__hash_4']?.()
  __napiInstance.exports['__napi_register__hash_sync_5']?.()
  __napiInstance.exports['__napi_register__RawHashTask_impl_6']?.()
  __napiInstance.exports['__napi_register__hash_raw_7']?.()
  __napiInstance.exports['__napi_register__hash_raw_sync_8']?.()
  __napiInstance.exports['__napi_register__VerifyTask_impl_9']?.()
  __napiInstance.exports['__napi_register__verify_10']?.()
  __napiInstance.exports['__napi_register__verify_sync_11']?.()
}
export const Algorithm = __napiModule.exports.Algorithm
export const hash = __napiModule.exports.hash
export const hashRaw = __napiModule.exports.hashRaw
export const hashRawSync = __napiModule.exports.hashRawSync
export const hashSync = __napiModule.exports.hashSync
export const verify = __napiModule.exports.verify
export const verifySync = __napiModule.exports.verifySync
export const Version = __napiModule.exports.Version
