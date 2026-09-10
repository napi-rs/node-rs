# Migrating to bcrypt 2

Stored password hashes remain usable after migrating API calls. Verification continues to use the salt and cost embedded in each stored hash, including hashes created through older custom-salt bugs. Do not rewrite hashes, replace their prefixes, or reset passwords for this upgrade.

## Calls use options objects

| 1.x                                    | 2.x                                               |
| -------------------------------------- | ------------------------------------------------- |
| `hash(password, 12)`                   | `hash(password, { cost: 12 })`                    |
| `hashSync(password, 12, rawSalt)`      | `hashSync(password, { cost: 12, salt: rawSalt })` |
| `genSalt(12, '2b', signal)`            | `genSalt({ cost: 12, version: '2b', signal })`    |
| `verify(password, storedHash, signal)` | `verify(password, storedHash, { signal })`        |
| `verify(password, storedHash)`         | Unchanged                                         |
| `compareSync(password, storedHash)`    | Unchanged                                         |

Omit unused options instead of passing `null`. Unsupported positional arguments, unknown options, and a bare signal where options are expected fail explicitly. All async validation errors now reject the returned Promise, so use `await` inside `try/catch` or attach `.catch()`.

## Salt creation is corrected

Raw salts must be exactly 16 bytes. String salts must be canonical encoded salts containing their cost and version, for example the result of `genSalt({ cost: 12 })`. Do not also specify cost/version when supplying an encoded salt. Generated salts now contain 29 characters without `==` padding.

Old versions treated string salts as raw text and clipped or zero-padded them. Correct interpretation intentionally changes newly computed output. Applications that authenticate by recomputing a hash from a separately saved original salt should switch to `verify(password, storedHash)`. The stored hash contains the actual salt used previously; it needs no conversion. Automatic random salts are the default for new hashes.

Costs must be finite integers in 4–31. Fractional and overflowing values are rejected instead of truncated or wrapped. Existing hashes still use their embedded effective costs; there is no new default verification cost ceiling.

Creation no longer accepts `2x`, including inside encoded salt strings. Verification retains its previous prefix handling. That existing behavior does not implement the historical sign-extension algorithm of genuine `2x` hashes; this release neither reinterprets those hashes nor tries multiple algorithms.

## Password bytes and login compatibility

Default 72-byte truncation remains in both hash creation and verification. This preserves existing logins and ordinary rehash-on-login flows. No compatibility flag is necessary. UTF-8 string encoding, raw byte inputs, embedded NULs, and empty passwords keep their previous meaning.

`rejectLongPasswords: true` is an optional creation policy. It rejects more than 72 bytes and accepts exactly 72. Enabling it can affect enrollment or rehashing of long passwords; verification never adopts it automatically. Stored bcrypt strings do not record whether the original password was truncated.

Invalid UTF-8 bytes supplied as the stored hash now return `false`, consistently with other malformed hash data. Successfully accepted noncanonical encodings, including the `+4` cost spelling, remain accepted by verification but are rejected for new salt creation.

## Cancellation and byte ownership

Async calls copy mutable byte inputs before returning. Changing a password, raw salt, or stored-hash array afterward no longer changes the queued operation.

Put `signal` in async options. A pre-aborted signal rejects before native work is queued. Later abort rejects the pending public Promise with `AbortError`; queued work is cancelled where possible, while running native computation may finish with its result discarded. Existing signal handlers are preserved, shared/reused signals work independently, and abort after observed completion has no effect.

Native signals and compatible signals from locally imported polyfills are accepted. On Node 10 and 12, import an `AbortController` polyfill and pass `controller.signal`; neither constructor needs to be installed globally.

Install the matching 2.x platform packages together with the root package. A backend contract check rejects stale binaries rather than silently interpreting new calls with old native arguments.
