# `@node-rs/bcrypt`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)

🚀 Fastest bcrypt in Node.js

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

## Usage

```typescript
export const DEFAULT_COST: 12

export function hashSync(password: string | Buffer, round?: number): string
export function hash(password: string | Buffer, round?: number): Promise<string>
export function verifySync(password: string | Buffer, hash: string | Buffer): boolean
export function verify(password: string | Buffer, hash: string | Buffer): Promise<boolean>
/**
 * The same with `verifySync`
 */
export function compareSync(password: string | Buffer, hash: string | Buffer): boolean
/**
 * The same with `verify`
 */
export function compare(password: string | Buffer, hash: string | Buffer): Promise<boolean>

export type Version = '2a' | '2x' | '2y' | '2b'
/**
 * @param version default '2b'
 */
export function genSaltSync(round: number, version?: Version): string
/**
 * @param version default '2b'
 */
export function genSalt(round: number, version?: Version): Promise<string>
```

## Bench

```
Model Name: MacBook Pro
Model Identifier: MacBookPro15,1
Processor Name: Intel Core i7
Processor Speed: 2.6 GHz
Number of Processors: 1
Total Number of Cores: 6
L2 Cache (per Core): 256 KB
L3 Cache: 12 MB
Hyper-Threading Technology: Enabled
Memory: 16 GB
```

```text
@node-rs/bcrypt x 18.55 ops/sec ±1.51% (12 runs sampled)
node bcrypt x 16.37 ops/sec ±2.94% (11 runs sampled)
bcryptjs x 3.61 ops/sec ±4.86% (6 runs sampled)
Async hash round 12 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 18.51 ops/sec ±1.60% (12 runs sampled)
node bcrypt x 16.51 ops/sec ±1.51% (11 runs sampled)
bcryptjs x 3.71 ops/sec ±2.23% (6 runs sampled)
Async verify bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 4.68 ops/sec ±4.69% (16 runs sampled)
node bcrypt x 3.94 ops/sec ±6.56% (14 runs sampled)
bcryptjs x 3.56 ops/sec ±2.04% (13 runs sampled)
Hash round 12 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 521,917 ops/sec ±2.27% (82 runs sampled)
node bcrypt x 252,333 ops/sec ±2.05% (82 runs sampled)
bcryptjs x 110,578 ops/sec ±2.37% (82 runs sampled)
genSaltSync bench suite: Fastest is @node-rs/bcrypt
```
