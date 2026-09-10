// Shared by the Node entry (including WASI fallback) and the browser entry.
// Keep this adapter parseable on Node 10, before any runtime feature checks.
module.exports = function createBcrypt(binding) {
  if (binding.BCRYPT_API_VERSION !== 2) {
    throw new Error('Incompatible bcrypt binary: rebuild or reinstall the matching @node-rs/bcrypt backend')
  }

  function arity(args, maximum) {
    if (args.length > maximum) throw new TypeError('Positional bcrypt options are no longer supported')
  }

  function options(value, keys) {
    if (value === undefined) return Object.create(null)
    if (value === null || typeof value !== 'object') throw new TypeError('options must be an object')
    const prototype = Object.getPrototypeOf(value)
    if (prototype !== null && prototype !== Object.prototype) throw new TypeError('options must be a plain object')
    const result = Object.create(null)
    for (const key of Reflect.ownKeys(value)) {
      if (!keys.includes(key)) throw new TypeError(`Unknown bcrypt option: ${String(key)}`)
      result[key] = value[key]
    }
    return result
  }

  function bytes(value, name) {
    if (typeof value === 'string') return value
    if (ArrayBuffer.isView(value) && Object.prototype.toString.call(value) === '[object Uint8Array]') return value
    throw new TypeError(`${name} must be a string or Uint8Array`)
  }

  function creation(value) {
    if (
      value.cost !== undefined &&
      (typeof value.cost !== 'number' || !Number.isInteger(value.cost) || value.cost < 4 || value.cost > 31)
    ) {
      throw new RangeError('cost must be an integer between 4 and 31')
    }
    if (value.version !== undefined && !['2a', '2b', '2y'].includes(value.version)) {
      throw new RangeError('version must be 2a, 2b, or 2y')
    }
    if (value.salt !== undefined) {
      bytes(value.salt, 'salt')
      if (typeof value.salt === 'string') {
        if (value.cost !== undefined || value.version !== undefined)
          throw new RangeError('an encoded salt already supplies cost and version')
      } else if (value.salt.byteLength !== 16) {
        throw new RangeError('raw salt must contain exactly 16 bytes')
      }
    }
    if (value.rejectLongPasswords !== undefined && typeof value.rejectLongPasswords !== 'boolean') {
      throw new TypeError('rejectLongPasswords must be a boolean')
    }
    return value
  }

  function signal(value) {
    if (value === undefined) return value
    const descriptor =
      typeof AbortSignal === 'undefined' ? undefined : Object.getOwnPropertyDescriptor(AbortSignal.prototype, 'aborted')
    const getter = descriptor && descriptor.get
    try {
      if (!getter) throw new TypeError()
      getter.call(value)
    } catch {
      throw new TypeError('signal must be an AbortSignal')
    }
    return value
  }

  function nativeError(error) {
    return error && error.code === 'InvalidArg' ? new RangeError(error.message) : error
  }

  function sync(start) {
    try {
      return start()
    } catch (error) {
      throw nativeError(error)
    }
  }

  function run(userSignal, start) {
    return new Promise((resolve, reject) => {
      // One private signal per task avoids the native adapter's shared-signal state
      // and prevents it from overwriting userSignal.onabort.
      const controller = userSignal === undefined ? undefined : new AbortController()
      let settled = false
      const finish = (callback, value) => {
        if (settled) return
        settled = true
        if (userSignal !== undefined) userSignal.removeEventListener('abort', abort)
        callback(value)
      }
      const abort = () => {
        if (settled) return
        const error = new Error('The operation was aborted')
        error.name = 'AbortError'
        finish(reject, error)
        controller.abort()
      }
      if (userSignal !== undefined) {
        userSignal.addEventListener('abort', abort, { once: true })
        if (userSignal.aborted) {
          abort()
          return
        }
      }
      try {
        // The binding copies all byte inputs before returning this Promise.
        const task = start(controller === undefined ? undefined : controller.signal)
        Promise.resolve(task).then(
          (value) => finish(resolve, value),
          (error) => finish(reject, nativeError(error)),
        )
      } catch (error) {
        finish(reject, nativeError(error))
      }
    })
  }

  function genSaltSync(value) {
    arity(arguments, 1)
    const opts = creation(options(value, ['cost', 'version']))
    return sync(() => binding.genSaltSync(opts.cost === undefined ? binding.DEFAULT_COST : opts.cost, opts.version))
  }

  async function genSalt(value) {
    arity(arguments, 1)
    const opts = creation(options(value, ['cost', 'version', 'signal']))
    return run(signal(opts.signal), (internal) =>
      binding.genSalt(opts.cost === undefined ? binding.DEFAULT_COST : opts.cost, opts.version, internal),
    )
  }

  function hashSync(password, value) {
    arity(arguments, 2)
    bytes(password, 'password')
    const opts = creation(options(value, ['cost', 'salt', 'version', 'rejectLongPasswords']))
    return sync(() => binding.hashSync(password, opts.cost, opts.salt, opts.version, opts.rejectLongPasswords === true))
  }

  async function hash(password, value) {
    arity(arguments, 2)
    bytes(password, 'password')
    const opts = creation(options(value, ['cost', 'salt', 'version', 'rejectLongPasswords', 'signal']))
    return run(signal(opts.signal), (internal) =>
      binding.hash(password, opts.cost, opts.salt, opts.version, opts.rejectLongPasswords === true, internal),
    )
  }

  function verifySync(password, encoded) {
    arity(arguments, 2)
    bytes(password, 'password')
    bytes(encoded, 'hash')
    return binding.verifySync(password, encoded)
  }

  async function verify(password, encoded, value) {
    arity(arguments, 3)
    bytes(password, 'password')
    bytes(encoded, 'hash')
    const opts = options(value, ['signal'])
    return run(signal(opts.signal), (internal) => binding.verify(password, encoded, internal))
  }

  return {
    DEFAULT_COST: binding.DEFAULT_COST,
    genSalt,
    genSaltSync,
    hash,
    hashSync,
    verify,
    verifySync,
    compare: verify,
    compareSync: verifySync,
  }
}
