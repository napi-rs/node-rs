# `@node-rs/jieba`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/jieba.svg?sanitize=true)

[jieba-rs](https://github.com/messense/jieba-rs) binding to Node.js

## Without node-gyp

`node-rs/jieba` was prebuilt into binary already, so you don't need fighting with `node-gyp` and c++ toolchain.

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

```javascript
const { load, cut } = require('@node-rs/jieba')

load()
// loadDict(fs.readFileSync(...))
// loadTFIDFDict(fs.readFileSync(...))

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

// [
//   { keyword: '北京烤鸭', weight: 1.3904870323222223 },
//   { keyword: '纽约', weight: 1.121759684755 },
//   { keyword: '天气', weight: 1.0766573240983333 }
// ]
```

### Load custom dictionaries

```javascript
const { loadDict, cut } = require('@node-rs/jieba')
const customDict = ['哪行 50', '干一行 51', '行一行 52', '行行 53']

const dictBuffer = Buffer.from(customDict.join('\n'), 'utf-8')
// loadDict doc: https://github.com/fxsjy/jieba?tab=readme-ov-file#%E8%BD%BD%E5%85%A5%E8%AF%8D%E5%85%B8
loadDict(dictBuffer)

const text = '人要是行干一行行一行，一行行行行行，行行行干哪行都行'
const output = cut(text, false)
console.log('分词结果⤵️\n', output.join('/'))
// Before: 人/要是/行/干/一行行/一行/，/一行行/行/行/行/，/行/行/行/干/哪/行/都行
// After:  人/要是/行/干一行/行一行/，/一行行/行行/行/，/行行/行/干/哪行/都行
// Pinyin: rén yào shi xíng gàn yì háng xíng yì háng ， yì háng xíng háng háng xíng ， háng háng xíng gàn nǎ háng dōu xíng
```
