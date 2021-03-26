# node-rs

When `Node.js` meet `Rust` = 🚀

# napi-rs

Make rust crates binding to Node.js use [napi-rs](https://github.com/napi-rs/napi-rs)

# Support matrix

|                 | node10 | node12 | node14 | node15 |
| --------------- | ------ | ------ | ------ | ------ |
| Windows x64     | ✓      | ✓      | ✓      | ✓      |
| Windows x32     | ✓      | ✓      | ✓      | ✓      |
| macOS x64       | ✓      | ✓      | ✓      | ✓      |
| macOS arm64     | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu   | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl  | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu   | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu | ✓      | ✓      | ✓      | ✓      |
| Android arm64   | ✓      | ✓      | ✓      | ✓      |

# Packages

| Package                                      | Status                                                         | Downloads                                                               | Description                                                        |
| -------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------ |
| [`@node-rs/crc32`](./packages/crc32)         | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/crc32.svg?sanitize=true)     | Fastest `CRC32` implementation using `SIMD`                        |
| [`@node-rs/jieba`](./packages/jieba)         | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/jieba.svg?sanitize=true)     | [`jieba-rs`](https://github.com/messense/jieba-rs) binding         |
| [`@node-rs/bcrypt`](./packages/bcrypt)       | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)    | Fastest bcrypt implementation                                      |
| [`@node-rs/deno-lint`](./packages/deno-lint) | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/deno-lint.svg?sanitize=true) | [deno_lint](https://github.com/denoland/deno_lint) Node.js binding |
