# `@node-rs/jsonwebtoken`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/jsonwebtoken.svg?sanitize=true)

🚀 Fastest jsonwebtoken in Node.js

## Support matrix

|                  | node14 | node16 | node18 | node20 |
| ---------------- | ------ | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      | ✓      |

## Usage

See [tests](__tests__/jsonwebtoken.spec.ts) and [types](index.d.ts)

## Bench

```
Model Name: MacBook Pro
Model Identifier: MacBookPro18,2
Processor Name: Apple M1 Max
Processor Speed: 2.6 GHz
Number of Processors: 1
Total Number of Cores: 8
L2 Cache (per Core): 256 KB
L3 Cache: 12 MB
Hyper-Threading Technology: Disabled
Memory: 64 GB
```

```text
@node-rs/jsonwebtoken x 17,491 ops/sec ±0.39% (92 runs sampled)
node-jsonwebtoken x 6,899 ops/sec ±0.62% (88 runs sampled)
Async sign bench suite: Fastest is @node-rs/jsonwebtoken

@node-rs/jsonwebtoken x 17,393 ops/sec ±1.57% (87 runs sampled)
node-jsonwebtoken x 5,256 ops/sec ±0.74% (85 runs sampled)
Async verify bench suite: Fastest is @node-rs/jsonwebtoken

@node-rs/jsonwebtoken x 264,001 ops/sec ±0.08% (101 runs sampled)
node-jsonwebtoken x 71,785 ops/sec ±1.01% (97 runs sampled)
Sync sign bench suite: Fastest is @node-rs/jsonwebtoken

@node-rs/jsonwebtoken x 278,657 ops/sec ±0.08% (99 runs sampled)
node-jsonwebtoken x 54,462 ops/sec ±0.53% (100 runs sampled)
Sync verify bench suite: Fastest is @node-rs/jsonwebtoken
```
