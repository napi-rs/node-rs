# `@node-rs/helper`

[![install size](https://packagephobia.com/badge?p=@node-rs/helper)](https://packagephobia.com/result?p=@node-rs/helper)
[![Downloads](https://img.shields.io/npm/dm/@node-rs/helper.svg?sanitize=true)](https://npmcharts.com/compare/@node-rs/helper?minimal=true)

> Helper library for load native package.

## Usage

### locateBinding

Load native binding file from `dirname`

```ts
loadBinding(dirname: string, filename?: string = 'index', packageName?: string): string
```

- `dirname`, dirname which the **.node** binding file located
- `filename`, the `napi.name` filed in you `package.json`
- `packageName`, the `name` filed in your `package.json`, `@swc/core` for example.

- return native module

```ts
const { loadBinding } = require('@node-rs/helper')

module.exports = loadBinding(__dirname, 'swc', '@swc/core')
```
