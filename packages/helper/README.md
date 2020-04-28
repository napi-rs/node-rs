# `@node-rs/helper`

> Helper library for node-rs

## Usage

### locateBinding

Load native binding file from `dirname`

```ts
locateBinding(dirname: string): string
```

- `dirname`, dirname which the **.node** binding file located
- return the full path of the binding file, throw if file not existed or platform not supported

```ts
const { locateBinding } = require('@node-rs/helper')

module.exports = require(locateBinding(__dirname))
```
