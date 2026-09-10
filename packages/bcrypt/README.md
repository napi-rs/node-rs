# `@node-rs/bcrypt`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)

🚀 Fastest bcrypt in Node.js

## Usage

```typescript
import { hash, hashSync, verify, verifySync, genSalt, compare } from '@node-rs/bcrypt'

const storedHash = await hash('password', { cost: 12 })
await verify('password', storedHash) // true

const salt = await genSalt({ cost: 12 })
const withExplicitSalt = hashSync('password', { salt })
verifySync('password', withExplicitSalt) // true
await compare('password', storedHash) // alias of verify
```

`hash` and `hashSync` accept a string or `Uint8Array` password and an options object. `cost` defaults to 12 and must be an integer from 4 through 31. Omitted salts use 16 random bytes. `salt` can be exactly 16 raw bytes or a canonical 29-character encoded bcrypt salt; an encoded salt supplies its own cost and version, so overrides are rejected. Creation supports `2a`, `2b` (default), and `2y`.

`genSalt` and `genSaltSync` accept `{ cost?, version? }`. `verify` and `verifySync` take the password first and the complete stored hash second. Both password and hash accept `Uint8Array`, including `Buffer`. `compare` and `compareSync` are exact aliases. See [the declarations](index.d.ts) for the complete API.

Bcrypt uses at most 72 password bytes. That default is unchanged for hashing and verification, including existing database hashes. To reject longer passwords when creating a hash, explicitly set `rejectLongPasswords: true`. This checks bytes, not JavaScript string length, and accepts exactly 72 bytes. Verification has no length-policy option.

Async functions accept `signal` inside their options object and report errors through Promise rejection. Synchronous functions throw. Invalid call shapes use `TypeError`; invalid creation values use `RangeError`. Wrong passwords and malformed stored hashes return `false`. Verification retains existing accepted encodings independently of the stricter creation parser.

```typescript
await hash('password', { cost: 12, signal: controller.signal })
await verify('password', storedHash, { signal: controller.signal })
```

An already-aborted signal prevents queueing. Aborting a pending operation rejects with `name: 'AbortError'`; native work that has already started may finish in the background. Shared and reused signals are supported without replacing existing handlers. The first observed completion or abort determines the result.

The browser entry uses the same public wrapper and aliases over WASI. Published packages include the matching WASI backend as an optional dependency. Platform-specific backend packages and `binding.js` are internal interfaces; import the public package entry.

Upgrading from 1.x requires call-site changes. **Existing stored hashes do not require rewriting or password resets.** See [migration instructions](MIGRATION.md).

## Bench

```
                  ,MMMM.           Host        -  xxxxxxxxxxxxxxxxxxxxxxx
                .MMMMMM            Machine     -  Mac15,9
                MMMMM,             Kernel      -  24.0.0
      .;MMMMM:' MMMMMMMMMM;.       OS          -  macOS 15.0.1 Sequoia
    MMMMMMMMMMMMNWMMMMMMMMMMM:     DE          -  Aqua
  .MMMMMMMMMMMMMMMMMMMMMMMMWM.     WM          -  Quartz Compositor
  MMMMMMMMMMMMMMMMMMMMMMMMM.       Packages    -  194 (Homebrew), 32 (cargo)
 ;MMMMMMMMMMMMMMMMMMMMMMMM:        Shell       -  zsh
 :MMMMMMMMMMMMMMMMMMMMMMMM:        Terminal    -  warpterminal (Version v0.2024.10.23.14.49.stable_00)
 .MMMMMMMMMMMMMMMMMMMMMMMMM.       Resolution  -  5120x2880@160fps (as 2560x1440)
  MMMMMMMMMMMMMMMMMMMMMMMMMMM.                    2992x1934@120fps (as 1496x967)
   .MMMMMMMMMMMMMMMMMMMMMMMMMM.                   2232x1512@60fps (as 1116x756)
     MMMMMMMMMMMMMMMMMMMMMMMM      Uptime      -  1d 2h 32m
      ;MMMMMMMMMMMMMMMMMMMM.       CPU         -  Apple M3 Max (16)
        .MMMM,.    .MMMM,.         CPU Load    -  16%
                                   Memory      -  50.1 GB / 134.2 GB
                                   Battery     -  78% & Discharging
                                   Disk Space  -  624.0 GB / 994.7 GB
```

```text
❯ yarn workspace @node-rs/bcrypt bench
Hash benchmark
┌─────────┬───────────────────┬─────────┬───────────────────┬──────────┬─────────┐
│ (index) │ Task Name         │ ops/sec │ Average Time (ns) │ Margin   │ Samples │
├─────────┼───────────────────┼─────────┼───────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/bcrypt' │ '20'    │ 49142200.63636367 │ '±1.08%' │ 11      │
│ 1       │ 'node bcrypt'     │ '20'    │ 49259219.81818187 │ '±1.35%' │ 11      │
│ 2       │ 'bcryptjs'        │ '17'    │ 58142116.79999998 │ '±0.36%' │ 10      │
│ 3       │ 'wasm OpenBSD'    │ '17'    │ 58318899.99999994 │ '±0.23%' │ 10      │
│ 4       │ 'wasm Openwall'   │ '18'    │ 53324629.20000016 │ '±0.37%' │ 10      │
└─────────┴───────────────────┴─────────┴───────────────────┴──────────┴─────────┘
Verify benchmark
┌─────────┬───────────────────┬─────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name         │ ops/sec │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼───────────────────┼─────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/bcrypt' │ '5'     │ 192946879.09999993 │ '±0.52%' │ 10      │
│ 1       │ 'node bcrypt'     │ '5'     │ 199806404.2        │ '±0.36%' │ 10      │
│ 2       │ 'bcryptjs'        │ '4'     │ 231329516.79999986 │ '±0.13%' │ 10      │
└─────────┴───────────────────┴─────────┴────────────────────┴──────────┴─────────┘
GenSalt benchmark
┌─────────┬───────────────────┬─────────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name         │ ops/sec     │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼───────────────────┼─────────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@node-rs/bcrypt' │ '4,421,897' │ 226.14727567195578 │ '±0.08%' │ 2210949 │
│ 1       │ 'node bcrypt'     │ '4,477,408' │ 223.34345972377827 │ '±0.08%' │ 2238705 │
│ 2       │ 'bcryptjs'        │ '821,556'   │ 1217.2015511950851 │ '±0.39%' │ 410779  │
│ 3       │ 'wasm OpenBSD'    │ '3,685,603' │ 271.3259889021118  │ '±0.23%' │ 1842802 │
│ 4       │ 'wasm Openwall'   │ '1,462,251' │ 683.8769076754866  │ '±2.50%' │ 731126  │
└─────────┴───────────────────┴─────────────┴────────────────────┴──────────┴─────────┘
```
