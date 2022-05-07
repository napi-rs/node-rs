# `@node-rs/xxhash`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/xxhash.svg?sanitize=true)
[![Install size](https://packagephobia.com/badge?p=@node-rs/xxhash)](https://packagephobia.com/result?p=@node-rs/xxhash)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

[`xxhash-rust`](https://github.com/DoumanAsh/xxhash-rust) binding for Node.js.

## Install this package

```
yarn add @node-rs/xxhash
pnpm add @node-rs/xxhash
npm install @node-rs/xxhash
```

## Support matrix

|                       | node12 | node14 | node16 | node18 |
| --------------------- | ------ | ------ | ------ | ------ |
| Windows x64           | ✓      | ✓      | ✓      | ✓      |
| Windows x32           | ✓      | ✓      | ✓      | ✓      |
| Windows arm64         | ✓      | ✓      | ✓      | ✓      |
| macOS x64             | ✓      | ✓      | ✓      | ✓      |
| macOS arm64 (m chips) | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu         | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl        | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu         | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu       | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 musl      | ✓      | ✓      | ✓      | ✓      |
| Android arm64         | ✓      | ✓      | ✓      | ✓      |
| Android armv7         | ✓      | ✓      | ✓      | ✓      |
| FreeBSD x64           | ✓      | ✓      | ✓      | ✓      |

## API

```ts
export type BufferLike =
  | Buffer
  | string
  | Uint8Array
  | ArrayBuffer
  | SharedArrayBuffer
  | ReadonlyArray<number>
  | number[]

export function xxh32(input: BufferLike, seed?: number): number
export function xxh64(input: BufferLike, seed?: BigInt): BigInt

export class Xxh32 {
  constructor(seed?: number)
  update(input: BufferLike): this
  digest(): number
  reset(): void
}

export class Xxh64 {
  constructor(seed?: BigInt)
  update(input: BufferLike): this
  digest(): BigInt
  reset(): void
}

export class Xxh3 {
  static withSeed(seed?: BigInt): Xxh3
  static withSecret(secret: BufferLike): Xxh3
  private constructor() {}
  update(input: BufferLike): this
  digest(): BigInt
  reset(): void
}

export const xxh3: {
  xxh64: (input: BufferLike, seed?: BigInt) => BigInt
  xxh64WithSecret: (input: BufferLike, secret: BufferLike) => BigInt
  xxh128: (input: BufferLike, seed?: BigInt) => BigInt
  xxh128WithSecret: (input: BufferLike, secret: BufferLike) => BigInt
  Xxh3: typeof Xxh3
}
```

## Performance

### Hardware

```
Model Name: MacBook Pro
Model Identifier: MacBookPro15,1
Processor Name: 6-Core Intel Core i7
Processor Speed: 2.6 GHz
Number of Processors: 1
Total Number of Cores: 6
L2 Cache (per Core): 256 KB
L3 Cache: 12 MB
Hyper-Threading Technology: Enabled
Memory: 16 GB
```

### Result

```
@node-rs/xxhash h32 x 18,847 ops/sec ±3.81% (81 runs sampled)
xxhash c++ x 12,190 ops/sec ±2.94% (83 runs sampled)
xxhashjs h32 x 1,035 ops/sec ±11.04% (68 runs sampled)
xxh32 bench suite: Fastest is @node-rs/xxhash h32

@node-rs/xxhash h32 x 13,248 ops/sec ±4.38% (78 runs sampled)
xxhashjs h32 x 1,366 ops/sec ±1.96% (85 runs sampled)
xxh32 multi steps bench suite: Fastest is @node-rs/xxhash h32

@node-rs/xxhash 64 x 43,532 ops/sec ±1.33% (88 runs sampled)
xxhash C++ x 41,658 ops/sec ±1.45% (90 runs sampled)
wasm x 32,415 ops/sec ±1.38% (90 runs sampled)
xxhashjs h64 x 47.52 ops/sec ±3.20% (62 runs sampled)
xxh64 bench suite: Fastest is @node-rs/xxhash 64

@node-rs/xxhash 64 x 33,153 ops/sec ±5.42% (76 runs sampled)
wasm x 29,477 ops/sec ±2.72% (81 runs sampled)
xxhashjs h64 x 54.96 ops/sec ±1.93% (71 runs sampled)
xxh64 multi steps bench suite: Fastest is @node-rs/xxhash 64
```
