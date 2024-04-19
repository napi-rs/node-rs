import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'

import __wasmUrl from './xxhash.wasm32-wasi.wasm?url'

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
  __napiInstance.exports['__napi_register__xxh32_0']?.()
  __napiInstance.exports['__napi_register__Xxh32_struct_1']?.()
  __napiInstance.exports['__napi_register__Xxh32_impl_6']?.()
  __napiInstance.exports['__napi_register__xxh64_7']?.()
  __napiInstance.exports['__napi_register__Xxh64_struct_8']?.()
  __napiInstance.exports['__napi_register__Xxh64_impl_13']?.()
  __napiInstance.exports['__napi_register__xxh64_14']?.()
  __napiInstance.exports['__napi_register__xxh64_with_secret_15']?.()
  __napiInstance.exports['__napi_register__xxh128_16']?.()
  __napiInstance.exports['__napi_register__xxh128_with_secret_17']?.()
  __napiInstance.exports['__napi_register__Xxh3_struct_18']?.()
  __napiInstance.exports['__napi_register__Xxh3_impl_24']?.()
}
export const Xxh32 = __napiModule.exports.Xxh32
export const Xxh64 = __napiModule.exports.Xxh64
export const xxh32 = __napiModule.exports.xxh32
export const xxh64 = __napiModule.exports.xxh64
export const xxh3 = __napiModule.exports.xxh3
