# `@node-rs/bcrypt`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)

🚀 Fastest bcrypt in NodeJS

## Support matrix

|                 | node10 | node12 | node14 | node15 |
| --------------- | ------ | ------ | ------ | ------ |
| Windows x64     | ✓      | ✓      | ✓      | ✓      |
| macOS x64/arm64 | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu   | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl  | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu   | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu | ✓      | ✓      | ✓      | ✓      |
| Android arm64   | ✓      | ✓      | ✓      | ✓      |

## Usage

```typescript
export const DEFAULT_ROUND = 12

function hashSync(password: string | Buffer, round?: number): string
function hash(password: string | Buffer, round?: number): Promise<string>
function verifySync(password: string | Buffer, hash: string | Buffer): boolean
function verify(password: string | Buffer, hash: string | Buffer): Promise<boolean>
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

```bash
@node-rs/bcrypt x 70.38 ops/sec ±1.40% (32 runs sampled)
node bcrypt x 63.88 ops/sec ±0.95% (30 runs sampled)
bcryptjs x 12.23 ops/sec ±4.90% (10 runs sampled)
Async hash round 10 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 16.55 ops/sec ±2.33% (11 runs sampled)
node bcrypt x 14.62 ops/sec ±1.48% (11 runs sampled)
bcryptjs x 3.12 ops/sec ±1.60% (6 runs sampled)
Async hash round 12 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 3.65 ops/sec ±5.08% (6 runs sampled)
node bcrypt x 3.50 ops/sec ±2.14% (6 runs sampled)
bcryptjs x 0.84 ops/sec ±12.31% (5 runs sampled)
Async hash round 14 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 18.78 ops/sec ±0.86% (12 runs sampled)
node bcrypt x 16.28 ops/sec ±3.55% (11 runs sampled)
bcryptjs x 3.70 ops/sec ±2.64% (6 runs sampled)
Async verify bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 19.24 ops/sec ±1.67% (52 runs sampled)
node bcrypt x 17.53 ops/sec ±0.38% (47 runs sampled)
bcryptjs x 14.71 ops/sec ±2.18% (41 runs sampled)
Hash round 10 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 4.79 ops/sec ±1.92% (16 runs sampled)
node bcrypt x 4.26 ops/sec ±2.43% (15 runs sampled)
bcryptjs x 3.76 ops/sec ±1.90% (14 runs sampled)
Hash round 12 bench suite: Fastest is @node-rs/bcrypt
@node-rs/bcrypt x 1.19 ops/sec ±2.27% (7 runs sampled)
node bcrypt x 1.00 ops/sec ±4.71% (7 runs sampled)
bcryptjs x 0.93 ops/sec ±5.60% (7 runs sampled)
Hash round 14 bench suite: Fastest is @node-rs/bcrypt
```
