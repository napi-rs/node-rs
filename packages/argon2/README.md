# `@node-rs/argon2`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/argon2.svg?sanitize=true)

[RustCrypto: Argon2](https://crates.io/crates/argon2) binding for Node.js.

Argon2 is a [key derivation function](https://en.wikipedia.org/wiki/Key_derivation_function) that was selected as the winner of the [Password Hashing Competition(PHC)](https://password-hashing.net) in July 2015.

Argon2 summarizes the state of the art in the design of memory-hard functions and can be used to hash passwords for credential storage, key derivation, or other applications.

It has a simple design aimed at the highest memory filling rate and effective use of multiple computing units, while still providing defense against tradeoff attacks (by exploiting the cache and memory organization of the recent processors).

## Features

- Faster performance.
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

## Support matrix

|                     | node12 | node14 | node16 | node18 |
| ------------------- | ------ | ------ | ------ | ------ |
| Windows x64         | ✓      | ✓      | ✓      | ✓      |
| Windows x32         | ✓      | ✓      | ✓      | ✓      |
| Windows arm64       | ✓      | ✓      | ✓      | ✓      |
| macOS x64           | ✓      | ✓      | ✓      | ✓      |
| macOS arm64(m chip) | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu       | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl      | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu       | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu     | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl    | ✓      | ✓      | ✓      | ✓      |
| Android arm64       | ✓      | ✓      | ✓      | ✓      |
| Android armv7       | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64         | ✓      | ✓      | ✓      | ✓      |

# Benchmarks

See [benchmark/](benchmark/argon2.js).

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
   * The default value is 4096, meaning a pool of 4 MiB per thread.
   */
  memoryCost?: number | undefined | null
  /**
   * The time cost is the amount of passes (iterations) used by the hash function. It increases hash strength at the cost of time required to compute.
   *
   * Value is an integer in decimal (1 to 10 digits), between 1 and (2^32)-1.
   *
   * The default value is 3.
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
