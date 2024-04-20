# Change Log

All notable changes to this project will be documented in this file.
See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## [0.5.3](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.5.2...@node-rs/jsonwebtoken@0.5.3) (2024-04-20)

### Bug Fixes

- **deps:** update dependency @napi-rs/wasm-runtime to ^0.2.0 ([#826](https://github.com/napi-rs/node-rs/issues/826)) ([f863999](https://github.com/napi-rs/node-rs/commit/f8639994082a42145a2a3a82f41daaf6eb9b5881))

## [0.5.2](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.5.1...@node-rs/jsonwebtoken@0.5.2) (2024-03-12)

**Note:** Version bump only for package @node-rs/jsonwebtoken

## [0.5.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.5.0...@node-rs/jsonwebtoken@0.5.1) (2024-03-01)

### Bug Fixes

- **jsonwebtoken:** handle error in decodeHeader ([#801](https://github.com/napi-rs/node-rs/issues/801)) ([edc6db8](https://github.com/napi-rs/node-rs/commit/edc6db8772b99080384a47d2546e9315adcfa4f4))

# [0.5.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.2.3...@node-rs/jsonwebtoken@0.5.0) (2024-02-29)

### Bug Fixes

- add browser entries for pkgs ([#760](https://github.com/napi-rs/node-rs/issues/760)) ([105ecad](https://github.com/napi-rs/node-rs/commit/105ecad99b5ce1a270b8e885e5a56c139db2f119))
- **crc32,denolint:** upgrade rust toolchain ([#787](https://github.com/napi-rs/node-rs/issues/787)) ([5586e4f](https://github.com/napi-rs/node-rs/commit/5586e4face711d9fd73f8f6b262d6a3537ce6ce0))
- **deps:** update rust crate jsonwebtoken to v9 ([1cd9d90](https://github.com/napi-rs/node-rs/commit/1cd9d90206039e354b10f8033ff40fb247ee9dee))
- **jsonwebtoken:** num typings ([#777](https://github.com/napi-rs/node-rs/issues/777)) ([0b265a9](https://github.com/napi-rs/node-rs/commit/0b265a955b55e013833d540b3f5861c4c20e993f))
- wasi browser compatible ([3ec8863](https://github.com/napi-rs/node-rs/commit/3ec88636fd32f5dc1357f7259267ff9823dfd80d))
- wasi package load logic ([#755](https://github.com/napi-rs/node-rs/issues/755)) ([505856c](https://github.com/napi-rs/node-rs/commit/505856c4f9cb4c1f07e008f7f0dee41e7285a817))

### Features

- **denolint:** upgrade denolint crate ([#762](https://github.com/napi-rs/node-rs/issues/762)) ([19b888c](https://github.com/napi-rs/node-rs/commit/19b888c2bd5c474c7ddded82e3e4dc680f056ef2))
- **jsonwebtoken:** expose decode header utility ([#795](https://github.com/napi-rs/node-rs/issues/795)) ([fc259ce](https://github.com/napi-rs/node-rs/commit/fc259ce0dc3feba9376bf5da2f8f494b84bb3de0))
- **jsonwebtoken:** flatten claims to align with rfc ([#780](https://github.com/napi-rs/node-rs/issues/780)) ([89f2ce3](https://github.com/napi-rs/node-rs/commit/89f2ce3e63e02123872bc8bc105060a4182e74d0))
- **jsonwebtoken:** support wasm32-wasi target ([#798](https://github.com/napi-rs/node-rs/issues/798)) ([b23382d](https://github.com/napi-rs/node-rs/commit/b23382db8a1e645f44d65f2232265e36eb6bc4c8))
- support wasi target on browser ([#757](https://github.com/napi-rs/node-rs/issues/757)) ([f39eec0](https://github.com/napi-rs/node-rs/commit/f39eec00c7322a26c1836cf1a19c11c9a9d53ef6))
- support wasi-wasm32 target ([#752](https://github.com/napi-rs/node-rs/issues/752)) ([12f25b5](https://github.com/napi-rs/node-rs/commit/12f25b5a5e09a01c832e4d26084acf4ddbd730b9))
- upgrade dependencies and lockfile ([#751](https://github.com/napi-rs/node-rs/issues/751)) ([70d7fa7](https://github.com/napi-rs/node-rs/commit/70d7fa72262c6e547950b30daa2d03583a1b04bd))

# [0.4.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.2.3...@node-rs/jsonwebtoken@0.4.0) (2024-01-09)

### Bug Fixes

- **deps:** update rust crate jsonwebtoken to v9 ([1cd9d90](https://github.com/napi-rs/node-rs/commit/1cd9d90206039e354b10f8033ff40fb247ee9dee))
- wasi package load logic ([#755](https://github.com/napi-rs/node-rs/issues/755)) ([505856c](https://github.com/napi-rs/node-rs/commit/505856c4f9cb4c1f07e008f7f0dee41e7285a817))

### Features

- support wasi target on browser ([#757](https://github.com/napi-rs/node-rs/issues/757)) ([f39eec0](https://github.com/napi-rs/node-rs/commit/f39eec00c7322a26c1836cf1a19c11c9a9d53ef6))
- support wasi-wasm32 target ([#752](https://github.com/napi-rs/node-rs/issues/752)) ([12f25b5](https://github.com/napi-rs/node-rs/commit/12f25b5a5e09a01c832e4d26084acf4ddbd730b9))
- upgrade dependencies and lockfile ([#751](https://github.com/napi-rs/node-rs/issues/751)) ([70d7fa7](https://github.com/napi-rs/node-rs/commit/70d7fa72262c6e547950b30daa2d03583a1b04bd))

# [0.3.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.2.3...@node-rs/jsonwebtoken@0.3.0) (2024-01-02)

### Bug Fixes

- **deps:** update rust crate jsonwebtoken to v9 ([1cd9d90](https://github.com/napi-rs/node-rs/commit/1cd9d90206039e354b10f8033ff40fb247ee9dee))

### Features

- support wasi-wasm32 target ([#752](https://github.com/napi-rs/node-rs/issues/752)) ([12f25b5](https://github.com/napi-rs/node-rs/commit/12f25b5a5e09a01c832e4d26084acf4ddbd730b9))
- upgrade dependencies and lockfile ([#751](https://github.com/napi-rs/node-rs/issues/751)) ([70d7fa7](https://github.com/napi-rs/node-rs/commit/70d7fa72262c6e547950b30daa2d03583a1b04bd))

## [0.2.3](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.2.2...@node-rs/jsonwebtoken@0.2.3) (2023-07-20)

### Bug Fixes

- update native packages binary meta ([e22b97f](https://github.com/napi-rs/node-rs/commit/e22b97f00c568d21a001df432136db51843edf80))

## [0.2.2](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.2.0...@node-rs/jsonwebtoken@0.2.2) (2023-07-20)

### Bug Fixes

- **bcrypt:** type ([a7b33e2](https://github.com/napi-rs/node-rs/commit/a7b33e2e9eee498a25bea34d6d95930d91aa7fd7))
- **jsonwebtoken:** make Claims.data optional ([08bb37f](https://github.com/napi-rs/node-rs/commit/08bb37fea139a5cceee2088e7a6595aeaae2b525))

## [0.2.1](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.2.0...@node-rs/jsonwebtoken@0.2.1) (2023-06-14)

### Bug Fixes

- **bcrypt:** type ([a7b33e2](https://github.com/napi-rs/node-rs/commit/a7b33e2e9eee498a25bea34d6d95930d91aa7fd7))
- **jsonwebtoken:** make Claims.data optional ([08bb37f](https://github.com/napi-rs/node-rs/commit/08bb37fea139a5cceee2088e7a6595aeaae2b525))

# [0.2.0](https://github.com/napi-rs/node-rs/compare/@node-rs/jsonwebtoken@0.1.0...@node-rs/jsonwebtoken@0.2.0) (2023-04-28)

### Features

- upgrade dependencies ([a0ef7de](https://github.com/napi-rs/node-rs/commit/a0ef7deb79e15dbe860c02fca21bc00dbc80de00))

# 0.1.0 (2023-03-02)

### Features

- **jsonwebtoken:** jsonwebtoken implementation ([70ceb14](https://github.com/napi-rs/node-rs/commit/70ceb14a0b56e8e8b610fc3a378c08dc36b6a0c7))
