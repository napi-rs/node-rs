import { verify, verifySync, type VerifyOptions } from '../index.js'

export function checkVerificationTypes(encoded: string, password: string, secret: Uint8Array, signal: AbortSignal) {
  const options: VerifyOptions = { secret }
  verify(encoded, password, options, signal)
  verifySync(encoded, password, options)
  // @ts-expect-error Hashing salt is not a verification option.
  verify(encoded, password, { salt: secret })
  // @ts-expect-error Encoded PHC parameters cannot be overridden.
  verifySync(encoded, password, { timeCost: 2 })
  // @ts-expect-error Argon2 retains its separate fourth-position signal.
  verify(encoded, password, { secret, signal })
}
