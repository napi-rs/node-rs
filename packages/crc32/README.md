# `@node-rs/crc32`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/crc32.svg?sanitize=true)

Fastest `crc32` implement in `Node.js`

The 4 tested implementations are:

- **@node-rs/crc32** Hardware accelerated CRC-32C from [crc32fast](https://crates.io/crates/crc32fast)
- **sse4_crc32c** Hardware accelerated CRC-32C from [sse4_crc32](https://github.com/Voxer/sse4_crc32)
- **js_crc32c** Javascript implemented CRC-32C
- **js_crc32** Javascript implemented CRC-32 from [buffer-crc32](https://github.com/brianloveswords/buffer-crc32)

## Performance

```bash
@node-rs/crc32 for inputs 1024B x 5,108,123 ops/sec ±1.86% (89 runs sampled)
@node-rs/crc32 for inputs 16931844B, avg 2066B x 271 ops/sec ±1.15% (85 runs sampled)
sse4_crc32c_hw for inputs 1024B x 3,543,443 ops/sec ±1.39% (93 runs sampled)
sse4_crc32c_hw for inputs 16931844B, avg 2066B x 209 ops/sec ±0.78% (76 runs sampled)
sse4_crc32c_sw for inputs 1024B x 1,460,284 ops/sec ±2.35% (90 runs sampled)
sse4_crc32c_sw for inputs 16931844B, avg 2066B x 93.50 ops/sec ±2.43% (69 runs sampled)
js_crc32c for inputs 1024B x 464,681 ops/sec ±0.46% (91 runs sampled)
js_crc32c for inputs 16931844B, avg 2066B x 28.25 ops/sec ±1.64% (51 runs sampled)
js_crc32 for inputs 1024B x 442,272 ops/sec ±2.66% (93 runs sampled)
js_crc32 for inputs 16931844B, avg 2066B x 22.12 ops/sec ±5.20% (40 runs sampled)
+---------------------+-------------------+----------------------+
|                     │ 1024B             │ 16931844B, avg 2066B |
+---------------------+-------------------+----------------------+
| @node-rs/crc32      │ 5,108,123 ops/sec │ 271 ops/sec          |
+---------------------+-------------------+----------------------+
| sse4_crc32c_hw      │ 3,543,443 ops/sec │ 209 ops/sec          |
+---------------------+-------------------+----------------------+
| sse4_crc32c_sw      │ 1,460,284 ops/sec │ 93.50 ops/sec        |
+---------------------+-------------------+----------------------+
| js_crc32c           │ 464,681 ops/sec   │ 28.25 ops/sec        |
+---------------------+-------------------+----------------------+
| js_crc32            │ 442,272 ops/sec   │ 22.12 ops/sec        |
+---------------------+-------------------+----------------------+
```

## Support matrix

|                  | node12 | node14 | node16 | node18 |
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

## API

> The type of **input initial crc** and **output crc number** is `u32`

```ts
export function crc32(input: Buffer, crc?: number): number
export function crc32c(input: Buffer, crc?: number): number
```

## Usage

```ts
const { crc32 } = require('@node-rs/crc32')
const { readFileSync } = require('fs')

const content = readFileSync('./avatar.png')

crc32(content)
```
