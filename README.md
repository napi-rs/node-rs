# node-rs

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)

When `Node.js` meet `Rust` = 🚀

# napi-rs

Make rust crates binding to Node.js use [napi-rs](https://github.com/napi-rs/napi-rs)

# Support matrix

|                              | node12 | node14 | node16 | node18 |
| ---------------------------- | ------ | ------ | ------ | ------ |
| Windows x64                  | ✓      | ✓      | ✓      | ✓      |
| Windows x32                  | ✓      | ✓      | ✓      | ✓      |
| Windows arm64                | ✓      | ✓      | ✓      | ✓      |
| macOS x64                    | ✓      | ✓      | ✓      | ✓      |
| macOS arm64 (m chips)        | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu (glibc 2.17)   | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl               | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu (glibc 2.17)   | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu (glibc 2.17) | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl             | ✓      | ✓      | ✓      | ✓      |
| Android arm64                | ✓      | ✓      | ✓      | ✓      |
| Android armv7                | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64                  | ✓      | ✓      | ✓      | ✓      |

# Packages

| Package                                            | Version                                                     | Downloads                                                                  | Description                                                                |
| -------------------------------------------------- | ----------------------------------------------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| [`@node-rs/crc32`](./packages/crc32)               | ![](https://img.shields.io/npm/v/@node-rs/crc32.svg)        | ![](https://img.shields.io/npm/dm/@node-rs/crc32.svg?sanitize=true)        | Fastest `CRC32` implementation using `SIMD`                                |
| [`@node-rs/jieba`](./packages/jieba)               | ![](https://img.shields.io/npm/v/@node-rs/jieba.svg)        | ![](https://img.shields.io/npm/dm/@node-rs/jieba.svg?sanitize=true)        | [`jieba-rs`](https://github.com/messense/jieba-rs) binding                 |
| [`@node-rs/bcrypt`](./packages/bcrypt)             | ![](https://img.shields.io/npm/v/@node-rs/bcrypt.svg)       | ![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)       | Fastest bcrypt implementation                                              |
| [`@node-rs/deno-lint`](./packages/deno-lint)       | ![](https://img.shields.io/npm/v/@node-rs/deno-lint.svg)    | ![](https://img.shields.io/npm/dm/@node-rs/deno-lint.svg?sanitize=true)    | [deno_lint](https://github.com/denoland/deno_lint) Node.js binding         |
| [`@node-rs/xxhash`](./packages/xxhash)             | ![](https://img.shields.io/npm/v/@node-rs/xxhash.svg)       | ![](https://img.shields.io/npm/dm/@node-rs/xxhash.svg?sanitize=true)       | [`xxhash-rust`](https://github.com/DoumanAsh/xxhash-rust) Node.js binding  |
| [`@node-rs/argon2`](./packages/argon2)             | ![](https://img.shields.io/npm/v/@node-rs/argon2.svg)       | ![](https://img.shields.io/npm/dm/@node-rs/argon2.svg?sanitize=true)       | [argon2](https://crates.io/crates/argon2) binding for Node.js.             |
| [`@node-rs/jsonwebtoken`](./packages/jsonwebtoken) | ![](https://img.shields.io/npm/v/@node-rs/jsonwebtoken.svg) | ![](https://img.shields.io/npm/dm/@node-rs/jsonwebtoken.svg?sanitize=true) | [jsonwebtoken](https://github.com/Keats/jsonwebtoken) binding for Node.js. |
