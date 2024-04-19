import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'

import __wasmUrl from './jieba.wasm32-wasi.wasm?url'

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
  __napiInstance.exports['__napi_register__load_0']?.()
  __napiInstance.exports['__napi_register__load_dict_1']?.()
  __napiInstance.exports['__napi_register__cut_2']?.()
  __napiInstance.exports['__napi_register__cut_all_3']?.()
  __napiInstance.exports['__napi_register__cut_for_search_4']?.()
  __napiInstance.exports['__napi_register__TaggedWord_struct_5']?.()
  __napiInstance.exports['__napi_register__tag_6']?.()
  __napiInstance.exports['__napi_register__Keyword_struct_7']?.()
  __napiInstance.exports['__napi_register__extract_8']?.()
  __napiInstance.exports['__napi_register__load_tfidf_dict_9']?.()
}
export const cut = __napiModule.exports.cut
export const cutAll = __napiModule.exports.cutAll
export const cutForSearch = __napiModule.exports.cutForSearch
export const extract = __napiModule.exports.extract
export const load = __napiModule.exports.load
export const loadDict = __napiModule.exports.loadDict
export const loadTFIDFDict = __napiModule.exports.loadTFIDFDict
export const tag = __napiModule.exports.tag
