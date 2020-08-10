# `@node-rs/bcrypt`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/bcrypt.svg?sanitize=true)

🚀 Fastest bcrypt in NodeJS

## Support matrix

|                   | node 10 | node12 | node14 |
| ----------------- | ------- | ------ | ------ |
| Windows 64 latest | ✓       | ✓      | ✓      |
| macOS latest      | ✓       | ✓      | ✓      |
| Linux             | ✓       | ✓      | ✓      |

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

<pre>
@node-rs/bcrypt x <span style="color: hotpink;">72.11</span> ops/sec ±1.43% (33 runs sampled)
node bcrypt x <span style="color: hotpink;">62.75</span> ops/sec ±2.95% (30 runs sampled)
Async hash round 10 bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
@node-rs/bcrypt x <span style="color: hotpink;">18.49</span> ops/sec ±1.04% (12 runs sampled)
node bcrypt x <span style="color: hotpink;">16.67</span> ops/sec ±2.05% (11 runs sampled)
Async hash round 12 bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
@node-rs/bcrypt x <span style="color: hotpink;">3.99</span> ops/sec ±3.17% (6 runs sampled)
node bcrypt x <span style="color: hotpink;">3.13</span> ops/sec ±1.92% (6 runs sampled)
Async hash round 14 bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
@node-rs/bcrypt x <span style="color: hotpink;">14.32</span> ops/sec ±0.55% (10 runs sampled)
node bcrypt x <span style="color: hotpink;">13.55</span> ops/sec ±2.83% (10 runs sampled)
Async verify bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
@node-rs/bcrypt x <span style="color: hotpink;">15.98</span> ops/sec ±1.12% (44 runs sampled)
node bcrypt x <span style="color: hotpink;">14.55</span> ops/sec ±1.30% (40 runs sampled)
Hash round 10 bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
@node-rs/bcrypt x <span style="color: hotpink;">4.65</span> ops/sec ±3.60% (16 runs sampled)
node bcrypt x <span style="color: hotpink;">4.26</span> ops/sec ±1.90% (15 runs sampled)
Hash round 12 bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
@node-rs/bcrypt x <span style="color: hotpink;">1.16</span> ops/sec ±2.65% (7 runs sampled)
node bcrypt x <span style="color: hotpink;">1.04</span> ops/sec ±2.95% (7 runs sampled)
Hash round 14 bench suite: Fastest is <span style="color: rgb(80, 250, 123);">@node-rs/bcrypt</span>
</pre>
