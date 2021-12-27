# `@node-rs/argon2`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/argon2.svg?sanitize=true)

[argon2](https://crates.io/crates/argon2) binding for Node.js.

## Support matrix

|                     | node12 | node14 | node16 | node17 |
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

## API

```typescript
export const enum Algorithm {
  Argon2d = 0,
  Argon2i = 1,
  Argon2id = 2,
}
export const enum Version {
  V0x10 = 0,
  V0x13 = 1,
}
export interface Options {
  /**
   * Memory size, expressed in kilobytes, between 1 and (2^32)-1.
   * Value is an integer in decimal (1 to 10 digits).
   */
  memoryCost?: number | undefined | null
  /**
   * Number of iterations, between 1 and (2^32)-1.
   * Value is an integer in decimal (1 to 10 digits).
   */
  timeCost?: number | undefined | null
  /**
   * Degree of parallelism, between 1 and 255.
   * Value is an integer in decimal (1 to 3 digits).
   */
  outputLen?: number | undefined | null
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
