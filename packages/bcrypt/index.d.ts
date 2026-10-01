export type Password = string | Uint8Array
/** Creation versions only; verification retains existing prefix handling independently. */
export type Version = '2a' | '2b' | '2y'
/** Prefixes the retained verifier accepts; imported `2x` labels are verified with the standard algorithm. */
export type StoredVersion = Version | '2x'
export declare const DEFAULT_COST: number // Remains 12.

/**
 * Error contract. Wrong argument or option types throw `TypeError` with
 * `code: 'ERR_INVALID_ARG_TYPE'`; values of the right type that are out of range
 * or malformed throw `RangeError` with `code: 'ERR_OUT_OF_RANGE'`; cancellation
 * rejects with `name: 'AbortError'` and `code: 'ABORT_ERR'`, like Node's AbortError.
 */
export type ErrorCode = 'ERR_INVALID_ARG_TYPE' | 'ERR_OUT_OF_RANGE' | 'ABORT_ERR'

/** The cancellation interface used from native AbortSignals and compatible polyfills. */
export interface AbortSignalLike {
  readonly aborted: boolean
  /** When defined at abort time, becomes the `cause` of the AbortError. */
  readonly reason?: unknown
  addEventListener(type: 'abort', listener: () => void, options?: { once?: boolean; capture?: boolean }): void
  /** Receives the same options object the listener was added with. */
  removeEventListener(type: 'abort', listener: () => void, options?: { once?: boolean; capture?: boolean }): void
}

export interface AsyncOptions {
  /**
   * For a valid call: pre-aborted signals reject with name AbortError before queueing.
   * Later abort rejects the pending public Promise with AbortError; running native work
   * may finish in the background. The error's `cause` is `signal.reason` when defined,
   * e.g. a TimeoutError from `AbortSignal.timeout()`. First observed settlement wins. Signal handlers
   * are preserved; shared/reused signals work independently for each operation.
   * Locally imported polyfills work without installing global constructors.
   */
  signal?: AbortSignalLike
}

export interface SaltOptions {
  /** Finite integer 4..31; defaults to 12. */
  cost?: number
  /** Defaults to 2b. 2x generation is removed. */
  version?: Version
}

export type HashOptions = (
  | { salt?: Uint8Array; cost?: number; version?: Version }
  | { salt: string; cost?: never; version?: never }
) & {
  /** Defaults to false. Optional creation policy; never imposed on verification. */
  rejectLongPasswords?: boolean
}

/** Canonical 29-character salt; async failures reject the returned Promise. */
export declare function genSalt(options?: SaltOptions & AsyncOptions): Promise<string>
export declare function genSaltSync(options?: SaltOptions): string

/**
 * Omitted salt generates 16 random bytes. Raw salts must contain exactly 16 bytes.
 * String salts: exactly 29 ASCII characters, prefix 2a/2b/2y, two decimal cost
 * digits in 04..31, and canonical bcrypt Base64 encoding exactly 16 salt bytes.
 * Embedded 2x prefixes, +4 costs, padding, and noncanonical trailing bits are rejected
 * during creation. Encoded salts supply their own cost/version.
 * No positional legacy overload or implicit salt clipping/padding remains.
 * Passwords retain existing bcrypt byte/truncation semantics by default.
 */
export declare function hash(password: Password, options?: HashOptions & AsyncOptions): Promise<string>
export declare function hashSync(password: Password, options?: HashOptions): string

/**
 * Same password bytes and stored hash retain their previous verification result.
 * No new length restriction, default cost ceiling, normalization, or prefix computation.
 * Async argument errors reject. Existing successfully parsed encodings stay supported,
 * including cost +4; creation's strict parser must not gate verification.
 * Encodings rejected by the retained verifier parser and password mismatches return false.
 * Invalid UTF-8 hash bytes also return false under the new error contract.
 * Caller options cannot override the stored salt, cost, or version.
 */
export declare function verify(password: Password, encodedHash: string | Uint8Array, options?: AsyncOptions): Promise<boolean>
export declare function verifySync(password: Password, encodedHash: string | Uint8Array): boolean
export declare const compare: typeof verify
export declare const compareSync: typeof verifySync

/** Parameters a stored hash was created with, as read by the verifier's parser. */
export interface ParsedHashOptions {
  version: StoredVersion
  /** Effective cost, 4..31. Noncanonical spellings such as `+4` are reported as their value. */
  cost: number
}

/**
 * Reads the version and cost of a stored hash with the same parser `verify` uses, so every hash
 * `verify` can accept is parseable. Compare the result against your current policy for
 * rehash-on-login checks. Hashes `verify` always rejects (malformed text, invalid UTF-8 bytes,
 * costs outside 4..31) throw `RangeError` instead of returning unusable parameters.
 * The stricter creation parser is not involved.
 */
export declare function parseOptions(encodedHash: string | Uint8Array): ParsedHashOptions
