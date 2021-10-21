# `@node-rs/xxhash`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/xxhash.svg?sanitize=true)
[![Install size](https://packagephobia.com/badge?p=@node-rs/xxhash)](https://packagephobia.com/result?p=@node-rs/xxhash)

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

Fastest `xxhash` implementation in Node.js.

## Install this package

```
yarn add @node-rs/xxhash
pnpm add @node-rs/xxhash
npm install @node-rs/xxhash
```

## Support matrix

|                       | node12 | node14 | node16 |
| --------------------- | ------ | ------ | ------ |
| Windows x64           | ✓      | ✓      | ✓      |
| Windows x32           | ✓      | ✓      | ✓      |
| Windows arm64         | ✓      | ✓      | ✓      |
| macOS x64             | ✓      | ✓      | ✓      |
| macOS arm64 (m chips) | ✓      | ✓      | ✓      |
| Linux x64 gnu         | ✓      | ✓      | ✓      |
| Linux x64 musl        | ✓      | ✓      | ✓      |
| Linux arm gnu         | ✓      | ✓      | ✓      |
| Linux arm64 gnu       | ✓      | ✓      | ✓      |
| Linux arm64 musl      | ✓      | ✓      | ✓      |
| Android arm64         | ✓      | ✓      | ✓      |
| FreeBSD x64           | ✓      | ✓      | ✓      |

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
}

export class Xxh64 {
  constructor(seed?: BigInt)
  update(input: BufferLike): this
  digest(): BigInt
}
```

## Performance

### Hardware

```
OS: Windows 10 x86_64
Host: Micro-Star International Co., Ltd. MS-7C35
Kernel: 10.0.19043
Terminal: Windows Terminal
CPU: AMD Ryzen 9 5950X (32) @ 3.400GHz
Memory: 32688MiB
```

### Result

```
@node-rs/xxhash h32 x 4,663 ops/sec ±6.22% (81 runs sampled)
xxhashjs h32 x 1,880 ops/sec ±7.11% (75 runs sampled)
xxh32 bench suite: Fastest is @node-rs/xxhash h32

@node-rs/xxhash h32 x 13,452 ops/sec ±2.73% (80 runs sampled)
xxhashjs h32 x 2,496 ops/sec ±0.39% (97 runs sampled)
xxh32 multi steps bench suite: Fastest is @node-rs/xxhash h32

@node-rs/xxhash 64 x 15,806 ops/sec ±3.14% (79 runs sampled)
xxhashjs h64 x 69.11 ops/sec ±5.99% (60 runs sampled)
xxh64 bench suite: Fastest is @node-rs/xxhash 64

@node-rs/xxhash 64 x 13,841 ops/sec ±3.17% (82 runs sampled)
xxhashjs h64 x 79.71 ops/sec ±4.34% (70 runs sampled)
xxh64 multi steps bench suite: Fastest is @node-rs/xxhash 64
```
