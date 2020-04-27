# `@node-rs/crc32`

Fastest `crc32` implement in `NodeJS`

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
