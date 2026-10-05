// Shared by the Node entry (including WASI fallback) and the browser entry.
// Keep this adapter parseable on Node 10, before any runtime feature checks.
module.exports = function createBcrypt(binding) {
  // Error codes follow Node's conventions so callers can branch without matching messages.
  function withCode(error, code) {
    error.code = code
    return error
  }
  const invalidType = (message) => withCode(new TypeError(message), 'ERR_INVALID_ARG_TYPE')
  const outOfRange = (message) => withCode(new RangeError(message), 'ERR_OUT_OF_RANGE')

  if (binding.BCRYPT_API_VERSION !== 2) {
    throw withCode(
      new Error('Incompatible bcrypt binary: rebuild or reinstall the matching @node-rs/bcrypt backend'),
      'ERR_BCRYPT_INCOMPATIBLE_BINARY',
    )
  }

  function arity(args, maximum, message = 'Positional bcrypt options are no longer supported') {
    if (args.length > maximum) throw invalidType(message)
  }

  function options(value, keys) {
    if (value === undefined) return Object.create(null)
    if (value === null || typeof value !== 'object') throw invalidType('options must be an object')
    const prototype = Object.getPrototypeOf(value)
    if (prototype !== null && prototype !== Object.prototype) throw invalidType('options must be a plain object')
    const result = Object.create(null)
    for (const key of Reflect.ownKeys(value)) {
      if (!keys.includes(key)) throw invalidType(`Unknown bcrypt option: ${String(key)}`)
      result[key] = value[key]
    }
    return result
  }

  function bytes(value, name) {
    if (typeof value === 'string') return value
    if (ArrayBuffer.isView(value) && Object.prototype.toString.call(value) === '[object Uint8Array]') return value
    throw invalidType(`${name} must be a string or Uint8Array`)
  }

  function creation(value) {
    if (value.cost !== undefined) {
      if (typeof value.cost !== 'number') throw invalidType('cost must be a number')
      if (!Number.isInteger(value.cost) || value.cost < 4 || value.cost > 31) {
        throw outOfRange('cost must be an integer between 4 and 31')
      }
    }
    if (value.version !== undefined) {
      if (typeof value.version !== 'string') throw invalidType('version must be a string')
      if (!['2a', '2b', '2y'].includes(value.version)) throw outOfRange('version must be 2a, 2b, or 2y')
    }
    if (value.salt !== undefined) {
      bytes(value.salt, 'salt')
      if (typeof value.salt === 'string') {
        if (value.cost !== undefined || value.version !== undefined)
          throw outOfRange('an encoded salt already supplies cost and version')
      } else if (value.salt.byteLength !== 16) {
        throw outOfRange('raw salt must contain exactly 16 bytes')
      }
    }
    if (value.rejectLongPasswords !== undefined && typeof value.rejectLongPasswords !== 'boolean') {
      throw invalidType('rejectLongPasswords must be a boolean')
    }
    return value
  }

  function signal(value) {
    if (value === undefined) return value
    if (
      value === null ||
      typeof value !== 'object' ||
      typeof value.aborted !== 'boolean' ||
      typeof value.addEventListener !== 'function' ||
      typeof value.removeEventListener !== 'function'
    ) {
      throw invalidType('signal must provide aborted, addEventListener, and removeEventListener')
    }
    return value
  }

  function nativeError(error) {
    return error && error.code === 'InvalidArg' ? outOfRange(error.message) : error
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
      // The native binding installs an onabort callback on the object it receives.
      // Give each task a private bridge, independent of the caller's signal implementation.
      const nativeSignal = userSignal === undefined ? undefined : { aborted: false, onabort: undefined }
      // Pass the same options when removing, for EventTargets that read them as a capture flag.
      const listenerOptions = { once: true }
      let settled = false
      const finish = (callback, value) => {
        if (settled) return
        settled = true
        callback(value)
        if (userSignal === undefined) return
        try {
          userSignal.removeEventListener('abort', abort, listenerOptions)
        } catch {
          // The call has settled; a listener that cannot be removed is left as a no-op.
        }
      }
      const abort = () => {
        if (settled) return
        // Same name and code as Node's AbortError.
        const error = withCode(new Error('The operation was aborted'), 'ABORT_ERR')
        error.name = 'AbortError'
        let reason
        try {
          reason = userSignal.reason
        } catch {
          // An unreadable reason must not stop the abort.
        }
        // Matches the native Error cause option, which Node < 16.9 ignores.
        if (reason !== undefined) {
          Object.defineProperty(error, 'cause', { configurable: true, writable: true, value: reason })
        }
        finish(reject, error)
        nativeSignal.aborted = true
        if (typeof nativeSignal.onabort === 'function') nativeSignal.onabort()
      }
      if (userSignal !== undefined) {
        userSignal.addEventListener('abort', abort, listenerOptions)
        if (settled || userSignal.aborted) {
          abort()
          return
        }
      }
      try {
        // The binding copies all byte inputs before returning this Promise.
        const task = start(nativeSignal)
        Promise.resolve(task).then(
          (value) => finish(resolve, value),
          (error) => finish(reject, nativeError(error)),
        )
      } catch (error) {
        finish(reject, nativeError(error))
      }
    })
  }

  const GEN_SALT_KEYS = ['cost', 'version']
  // Frozen stand-in for the empty options object a missing argument produces;
  // nothing downstream ever mutates it.
  const EMPTY_GEN_SALT_OPTIONS = Object.freeze(Object.create(null))

  // True when options() would accept value unchanged: a plain object whose own
  // keys are all in keys. Such inputs are already the object options() would
  // build, so genSaltSync validates them in place and skips the copy.
  function reusableOptions(value, keys) {
    if (value === null || typeof value !== 'object') return false
    const prototype = Object.getPrototypeOf(value)
    if (prototype !== null && prototype !== Object.prototype) return false
    for (const key of Reflect.ownKeys(value)) {
      if (!keys.includes(key)) return false
    }
    return true
  }

  function genSaltSync(value) {
    arity(arguments, 1)
    // Hot shapes: {} and {cost} skip the Object.create(null) copy in options();
    // anything else falls through so the errors stay identical.
    const input =
      value === undefined
        ? EMPTY_GEN_SALT_OPTIONS
        : reusableOptions(value, GEN_SALT_KEYS)
          ? value
          : options(value, GEN_SALT_KEYS)
    const opts = creation(input)
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

  function parseOptions(encoded) {
    arity(arguments, 1, 'parseOptions accepts a single hash argument')
    bytes(encoded, 'hash')
    return sync(() => binding.parseOptions(encoded))
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
    parseOptions,
  }
}
