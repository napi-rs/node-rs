# Change Log

All notable changes to this project will be documented in this file.
See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## [1.10.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.10.0...@node-rs/jieba@1.10.1) (2024-04-20)

### Bug Fixes

- **deps:** update dependency @napi-rs/wasm-runtime to ^0.2.0 ([#826](https://github.com/napi-rs/node-rs/issues/826)) ([f863999](https://github.com/napi-rs/node-rs/commit/f8639994082a42145a2a3a82f41daaf6eb9b5881))

# [1.10.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.7.2...@node-rs/jieba@1.10.0) (2024-02-29)

### Bug Fixes

- add browser entries for pkgs ([#760](https://github.com/napi-rs/node-rs/issues/760)) ([105ecad](https://github.com/napi-rs/node-rs/commit/105ecad99b5ce1a270b8e885e5a56c139db2f119))
- **crc32,denolint:** upgrade rust toolchain ([#787](https://github.com/napi-rs/node-rs/issues/787)) ([5586e4f](https://github.com/napi-rs/node-rs/commit/5586e4face711d9fd73f8f6b262d6a3537ce6ce0))
- wasi browser compatible ([3ec8863](https://github.com/napi-rs/node-rs/commit/3ec88636fd32f5dc1357f7259267ff9823dfd80d))
- wasi package load logic ([#755](https://github.com/napi-rs/node-rs/issues/755)) ([505856c](https://github.com/napi-rs/node-rs/commit/505856c4f9cb4c1f07e008f7f0dee41e7285a817))

### Features

- **jsonwebtoken:** flatten claims to align with rfc ([#780](https://github.com/napi-rs/node-rs/issues/780)) ([89f2ce3](https://github.com/napi-rs/node-rs/commit/89f2ce3e63e02123872bc8bc105060a4182e74d0))
- support wasi target on browser ([#757](https://github.com/napi-rs/node-rs/issues/757)) ([f39eec0](https://github.com/napi-rs/node-rs/commit/f39eec00c7322a26c1836cf1a19c11c9a9d53ef6))
- support wasi-wasm32 target ([#752](https://github.com/napi-rs/node-rs/issues/752)) ([12f25b5](https://github.com/napi-rs/node-rs/commit/12f25b5a5e09a01c832e4d26084acf4ddbd730b9))
- upgrade dependencies and lockfile ([#751](https://github.com/napi-rs/node-rs/issues/751)) ([70d7fa7](https://github.com/napi-rs/node-rs/commit/70d7fa72262c6e547950b30daa2d03583a1b04bd))

# [1.9.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.7.2...@node-rs/jieba@1.9.0) (2024-01-09)

### Bug Fixes

- wasi package load logic ([#755](https://github.com/napi-rs/node-rs/issues/755)) ([505856c](https://github.com/napi-rs/node-rs/commit/505856c4f9cb4c1f07e008f7f0dee41e7285a817))

### Features

- support wasi target on browser ([#757](https://github.com/napi-rs/node-rs/issues/757)) ([f39eec0](https://github.com/napi-rs/node-rs/commit/f39eec00c7322a26c1836cf1a19c11c9a9d53ef6))
- support wasi-wasm32 target ([#752](https://github.com/napi-rs/node-rs/issues/752)) ([12f25b5](https://github.com/napi-rs/node-rs/commit/12f25b5a5e09a01c832e4d26084acf4ddbd730b9))
- upgrade dependencies and lockfile ([#751](https://github.com/napi-rs/node-rs/issues/751)) ([70d7fa7](https://github.com/napi-rs/node-rs/commit/70d7fa72262c6e547950b30daa2d03583a1b04bd))

# [1.8.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.7.2...@node-rs/jieba@1.8.0) (2024-01-02)

### Features

- support wasi-wasm32 target ([#752](https://github.com/napi-rs/node-rs/issues/752)) ([12f25b5](https://github.com/napi-rs/node-rs/commit/12f25b5a5e09a01c832e4d26084acf4ddbd730b9))
- upgrade dependencies and lockfile ([#751](https://github.com/napi-rs/node-rs/issues/751)) ([70d7fa7](https://github.com/napi-rs/node-rs/commit/70d7fa72262c6e547950b30daa2d03583a1b04bd))

## [1.7.2](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.7.1...@node-rs/jieba@1.7.2) (2023-07-20)

### Bug Fixes

- update native packages binary meta ([e22b97f](https://github.com/napi-rs/node-rs/commit/e22b97f00c568d21a001df432136db51843edf80))

## [1.7.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.7.0...@node-rs/jieba@1.7.1) (2023-07-20)

### Bug Fixes

- **bcrypt:** type ([a7b33e2](https://github.com/napi-rs/node-rs/commit/a7b33e2e9eee498a25bea34d6d95930d91aa7fd7))

# [1.7.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.6.2...@node-rs/jieba@1.7.0) (2023-04-28)

### Features

- upgrade dependencies ([a0ef7de](https://github.com/napi-rs/node-rs/commit/a0ef7deb79e15dbe860c02fca21bc00dbc80de00))

## [1.6.2](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.6.1...@node-rs/jieba@1.6.2) (2023-01-11)

**Note:** Version bump only for package @node-rs/jieba

## [1.6.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.6.0...@node-rs/jieba@1.6.1) (2022-05-14)

### Bug Fixes

- Node.js worker_threads safe by upgrade NAPI-RS version ([#623](https://github.com/napi-rs/node-rs/issues/623)) ([792f69d](https://github.com/napi-rs/node-rs/commit/792f69d7ac1055947ac47c8049f16c863d3a0ad8))

# [1.6.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.5.5...@node-rs/jieba@1.6.0) (2022-02-11)

### Features

- upgrade to napi@2.1.0 ([27d003b](https://github.com/napi-rs/node-rs/commit/27d003b28919ff5f499abe1d4bbd77cc5afb930d))

## [1.5.5](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.5.4...@node-rs/jieba@1.5.5) (2022-01-18)

**Note:** Version bump only for package @node-rs/jieba

## [1.5.4](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.5.3...@node-rs/jieba@1.5.4) (2022-01-13)

**Note:** Version bump only for package @node-rs/jieba

## [1.5.3](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.5.2...@node-rs/jieba@1.5.3) (2022-01-13)

**Note:** Version bump only for package @node-rs/jieba

## [1.5.2](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.5.1...@node-rs/jieba@1.5.2) (2022-01-10)

### Bug Fixes

- **argon2,bcrypt,crc32,deno-lint,jieba,xxhash:** update js binding ([4ac6d2b](https://github.com/napi-rs/node-rs/commit/4ac6d2b9e9072a63216d05b47c92d3725b5b36f4))

## [1.5.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.5.0...@node-rs/jieba@1.5.1) (2021-12-23)

**Note:** Version bump only for package @node-rs/jieba

# [1.5.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.4.1...@node-rs/jieba@1.5.0) (2021-12-22)

### Features

- upgrade napi-rs to v2 ([#561](https://github.com/napi-rs/node-rs/issues/561)) ([7914fd5](https://github.com/napi-rs/node-rs/commit/7914fd526b03b0bb22d06cfd18024ae41206040f))

## [1.4.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.4.0...@node-rs/jieba@1.4.1) (2021-10-22)

**Note:** Version bump only for package @node-rs/jieba

# [1.4.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.3.1...@node-rs/jieba@1.4.0) (2021-10-21)

### Features

- **jieba:** upgrade once_cell crate ([9ffdd4a](https://github.com/napi-rs/node-rs/commit/9ffdd4ac000be54dac070e05352237dec8beefb9))

## [1.3.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.3.0...@node-rs/jieba@1.3.1) (2021-07-22)

**Note:** Version bump only for package @node-rs/jieba

# [1.3.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.2.0...@node-rs/jieba@1.3.0) (2021-07-18)

### Features

- Switch to nightly toolchain, strip symbols ([#463](https://github.com/napi-rs/node-rs/pull/463))

# [1.2.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.1.1...@node-rs/jieba@1.2.0) (2021-06-08)

### Features

- **jieba:** implement loadTFIDFDict ([c434bff](https://github.com/napi-rs/node-rs/commit/c434bfffc904d3707b8fbc93befe67a6fc828cd7))

## [1.1.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.1.0...@node-rs/jieba@1.1.1) (2021-06-01)

**Note:** Version bump only for package @node-rs/jieba

# [1.1.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@1.0.0...@node-rs/jieba@1.1.0) (2021-02-01)

### Features

- support win32-i686 platform ([c0f2f62](https://github.com/napi-rs/node-rs/commit/c0f2f62adc1fae15263086781e34d78d8eeeaecc))

# [0.6.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@0.5.1...@node-rs/jieba@0.6.0) (2020-09-04)

**Note:** Version bump only for package @node-rs/jieba

# [0.4.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jieba@0.4.0-alpha.1...@node-rs/jieba@0.4.0) (2020-08-18)

### Features

- **jieba:** bump to stable jieba-rs ([f412b49](https://github.com/napi-rs/node-rs/commit/f412b49776091aa5713e2881fc88eafc5d647c82))
