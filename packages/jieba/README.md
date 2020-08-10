# `@node-rs/jieba`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/jieba.svg?sanitize=true)

[jieba-rs](https://github.com/messense/jieba-rs) binding to NodeJS

## Without node-gyp

`node-rs/jieba` was prebuilt into binary already, so you don't need fighting with `node-gyp` and c++ toolchains.

## Performance

Due to [jieba-rs is 33% faster than cppjieba](https://blog.paulme.ng/posts/2019-06-30-optimizing-jieba-rs-to-be-33percents-faster-than-cppjieba.html), and N-API is faster than `v8` C++ API, `@node-rs/jieba` is faster than `nodejieba`.

```bash
@node-rs/jieba x 3,763 ops/sec ±1.18% (92 runs sampled)
nodejieba x 2,783 ops/sec ±0.67% (91 runs sampled)
Cut 1184 words bench suite: Fastest is @node-rs/jieba

@node-rs/jieba x 16.10 ops/sec ±1.58% (44 runs sampled)
nodejieba x 9.81 ops/sec ±2.39% (29 runs sampled)
Cut 246568 words bench suite: Fastest is @node-rs/jieba

@node-rs/jieba x 1,739 ops/sec ±0.87% (92 runs sampled)
nodejieba x 931 ops/sec ±1.31% (89 runs sampled)
Tag 1184 words bench suite: Fastest is @node-rs/jieba

@node-rs/jieba x 6.19 ops/sec ±2.01% (20 runs sampled)
nodejieba x 3.06 ops/sec ±5.39% (12 runs sampled)
Tag 246568 words bench suite: Fastest is @node-rs/jieba
```

## Support matrix

|                   | node 10 | node12 | node14 |
| ----------------- | ------- | ------ | ------ |
| Windows 64 latest | ✓       | ✓      | ✓      |
| macOS latest      | ✓       | ✓      | ✓      |
| Linux             | ✓       | ✓      | ✓      |

## Usage

```javascript
const { load, cut } = require('@node-rs/jieba')

load()

cut('我们中出了一个叛徒', false)

// ["我们", "中", "出", "了", "一个", "叛徒"]
```

```javascript
const { load, cut } = require('@node-rs/jieba')

load()

extract(
  '今天纽约的天气真好啊，京华大酒店的张尧经理吃了一只北京烤鸭。后天纽约的天气不好，昨天纽约的天气也不好，北京烤鸭真好吃',
  3,
)

// ["北京烤鸭", "纽约", "天气"]
```
