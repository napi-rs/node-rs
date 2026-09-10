import binding from '@node-rs/bcrypt-wasm32-wasi'
import createBcrypt from './api.cjs'

// Keep the public adapter separate from browser.js, which napi build regenerates.
const api = createBcrypt(binding)
export const { DEFAULT_COST, genSalt, genSaltSync, hash, hashSync, verify, verifySync, compare, compareSync } = api
