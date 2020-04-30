# `@node-rs/crc32`

![](https://github.com/Brooooooklyn/node-rs/workflows/CI/badge.svg)

Fastest `crc32` implement in `NodeJS`

# Support matrix

|                   | node 10 | node12 | node13 | node14 |
| ----------------- | ------- | ------ | ------ | ------ |
| Windows 64 latest | ✅      | ✅     | ✅     | ✅     |
| macOS latest      | ✅      | ✅     | ✅     | ✅     |
| Linux             | ✅      | ✅     | ✅     | ✅     |

## API

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
