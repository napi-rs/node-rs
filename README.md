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

| Package                                | Status                                                         | Description                                                |
| -------------------------------------- | -------------------------------------------------------------- | ---------------------------------------------------------- |
| [`@node-rs/crc32`](./packages/crc32)   | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | Fastest `CRC32` implementation using `SIMD`                |
| [`@node-rs/jieba`](./packages/jieba)   | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | [`jieba-rs`](https://github.com/messense/jieba-rs) binding |
| [`@node-rs/bcrypt`](./packages/bcrypt) | ![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg) | Fastest bcrypt implementation                              |
