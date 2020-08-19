# node-rs

When `NodeJS` meet `Rust` = 🚀

# napi-rs

Make rust crates binding to NodeJS use [napi-rs](https://github.com/Brooooooklyn/napi-rs)

# Support matrix

|                   | node 10 | node12 | node14 |
| ----------------- | ------- | ------ | ------ |
| Windows 64 latest | ✅      | ✅     | ✅     |
| macOS latest      | ✅      | ✅     | ✅     |
| Linux             | ✅      | ✅     | ✅     |
| Linux musl        | ❌      | ✅     | ✅     |

# Packages

| Package                                      | Status                                                         | Downloads                                                               | Description                                                       |
| -------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------- |
| [`@node-rs/crc32`](./packages/crc32)         | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/crc32.svg?sanitize=true)     | Fastest `CRC32` implementation using `SIMD`                       |
| [`@node-rs/jieba`](./packages/jieba)         | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/jieba.svg?sanitize=true)     | [`jieba-rs`](https://github.com/messense/jieba-rs) binding        |
| [`@node-rs/bcrypt`](./packages/bcrypt)       | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)    | Fastest bcrypt implementation                                     |
| [`@node-rs/deno-lint`](./packages/deno-lint) | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | ![](https://img.shields.io/npm/dm/@node-rs/deno-lint.svg?sanitize=true) | [deno_lint](https://github.com/denoland/deno_lint) nodejs binding |
