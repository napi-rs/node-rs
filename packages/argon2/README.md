# `@node-rs/argon2`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/argon2.svg?sanitize=true)

[argon2-rust](https://crates.io/crates/argon2-rust) binding for Node.js.

Argon2 is a [key derivation function](https://en.wikipedia.org/wiki/Key_derivation_function) that was selected as the winner of the [Password Hashing Competition(PHC)](https://password-hashing.net) in July 2015.

Argon2 summarizes the state of the art in the design of memory-hard functions and can be used to hash passwords for credential storage, key derivation, or other applications.

It has a simple design aimed at the highest memory filling rate and effective use of multiple computing units, while still providing defense against tradeoff attacks (by exploiting the cache and memory organization of the recent processors).

## Features

- Faster than node-argon2 and `node:crypto.argon2` at the same m/t/p (see Benchmarks).
- No node-gyp and postinstall.
- Cross-platform support, including [Apple M1](https://www.apple.com/newsroom/2020/11/apple-unleashes-m1/).
- Smaller file size after npm installation(476K vs [node-argon2](https://github.com/ranisalt/node-argon2) 3.7M).
- `@node-rs/argon2` supports all three algorithms:
  - Argon2i: Optimizes against GPU cracking attacks but vulnerable to side-channels.
    Accesses the memory array in a password dependent order, reducing the possibility of time–memory tradeoff (TMTO) attacks.
  - Argon2d: Optimized to resist side-channel attacks.
    Accesses the memory array in a password independent order, increasing the possibility of time-memory tradeoff (TMTO) attacks.
  - **Argon2id**: default value, this is the default algorithm for normative recommendations.
    Hybrid that mixes Argon2i and Argon2d passes.
    Uses the Argon2i approach for the first half pass over memory and Argon2d approach for subsequent passes. This effectively places it in the “middle” between the other two: it doesn’t provide as good TMTO/GPU cracking resistance as Argon2d, nor as good of side-channel resistance as Argon2i, but overall provides the most well-rounded approach to both classes of attacks.

# Benchmarks

Comparing each library's **defaults** is not 1:1. `@node-rs/argon2` defaults to `m=19456,t=2,p=1`; `argon2` (node-argon2) defaults to `m=65536,t=3,p=4`. See [#841](https://github.com/napi-rs/node-rs/issues/841).

This bench pins the same password, salt, Argon2id v=19, m, t, p, and 32-byte tag. Only the **raw** KDF is timed. The tag is asserted equal on every run. Native impls are interleaved with each other; JS/wasm are a separate group so a 7 ms hash is not timed after a 400 ms JS loop.

Apple M5 Max / arm64 / Node 24 / [argon2-rust 1.1.0](https://crates.io/crates/argon2-rust). Median ms. See [benchmark/](benchmark/argon2.ts).

**Native async raw** — same calling shape as node-argon2 (no sync API there):

| params                | @node-rs/argon2 | node-argon2 | node:crypto |
| --------------------- | --------------: | ----------: | ----------: |
| m=19456 KiB, t=2, p=1 |        **7.58** |       15.41 |       13.05 |
| m=65536 KiB, t=3, p=1 |       **49.35** |       84.57 |       71.71 |
| m=65536 KiB, t=3, p=4 |       **13.62** |       22.54 |       20.83 |

**JS / wasm raw** — same params, same tags:

| params                | hash-wasm | @noble/hashes |
| --------------------- | --------: | ------------: |
| m=19456 KiB, t=2, p=1 |     19.79 |         86.14 |
| m=65536 KiB, t=3, p=1 |    104.09 |        433.07 |
| m=65536 KiB, t=3, p=4 |    106.06 |        435.39 |

hash-wasm and `@noble/hashes` do not run lanes in parallel, so their p=4 row is the p=1 work.

## API

```typescript
export const enum Algorithm {
  Argon2d = 0,
  Argon2i = 1,
  Argon2id = 2,
}
export const enum Version {
  /** Version 16 (0x10 in hex) */
  V0x10 = 0,
  /**
   * Default value
   * Version 19 (0x13 in hex, default)
   */
  V0x13 = 1,
}
export interface Options {
  /**
   * The amount of memory to be used by the hash function, in kilobytes. Each thread will have a memory pool of this size. Note that large values for highly concurrent usage will cause starvation and thrashing if your system memory gets full.
   *
   * Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
   *
   * The default value is 19456, meaning a pool of 19 MiB per thread.
   */
  memoryCost?: number | undefined | null
  /**
   * The time cost is the amount of passes (iterations) used by the hash function. It increases hash strength at the cost of time required to compute.
   *
   * Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
   *
   * The default value is 2.
   */
  timeCost?: number | undefined | null
  /**
   * The hash length is the length of the hash function output in bytes. Note that the resulting hash is encoded with Base 64, so the digest will be ~1/3 longer.
   *
   * The default value is 32, which produces raw hashes of 32 bytes or digests of 43 characters.
   */
  outputLen?: number | undefined | null
  /**
   * The amount of threads to compute the hash on. Each thread has a memory pool with memoryCost size. Note that changing it also changes the resulting hash.
   *
   * Value is an integer in decimal (1 to 3 digits), between 1 and 255.
   *
   * The default value is 1, meaning a single thread is used.
   */
  parallelism?: number | undefined | null
  algorithm?: Algorithm | undefined | null
  version?: Version | undefined | null
  secret?: Buffer | undefined | null
}
export function hash(
  password: string | Buffer,
  options?: Options | undefined | null,
  abortSignal?: AbortSignal | undefined | null,
): Promise<string>
export function verify(
  hashed: string | Buffer,
  password: string | Buffer,
  options?: Options | undefined | null,
  abortSignal?: AbortSignal | undefined | null,
): Promise<boolean>
```
