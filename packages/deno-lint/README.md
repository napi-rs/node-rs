# `@node-rs/deno-lint`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/deno-lint.svg?sanitize=true)

> deno_lint Node.js binding

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

## Performance

### Hardware info

```
Model Name: MacBook Pro
Model Identifier: MacBookPro15,1
Processor Name: 6-Core Intel Core i9
Processor Speed: 2.9 GHz
Number of Processors: 1
Total Number of Cores: 6
L2 Cache (per Core): 256 KB
L3 Cache: 12 MB
Hyper-Threading Technology: Enabled
Memory: 32 GB
```

### Benchmark

```
@node-rs/deno-lint x 885 ops/sec ±1.26% (92 runs sampled)
eslint x 118 ops/sec ±4.97% (78 runs sampled)
Lint benchmark bench suite: Fastest is @node-rs/deno-lint
```

## Usage

```ts
import { lint } from '@node-rs/deno-lint'

lint(filepath, source, enableAllRules, excludeRules, includeRules)
```

## webpack-loader

```js
// webpack.config.js

module.exports = {
  module: {
    rules: [
      {
        enforce: 'pre',
        test: /\.(t|j)s?$/,
        loader: '@node-rs/deno-lint/webpack-loader',
        exclude: [/node_modules/],
      },
    ],
  },
}
```

### Options

You can pass denolint options using standard webpack loader options.

#### `enableAllRules`

- Type: `Boolean`
- Default: `false`

Whether to enable all rules. If false, `denolint` will enable all recommend rules.

#### `excludeRules`

- Type: `String[]`
- Default: `[]`

Rules to exclude from all or recommended ones chosen by `enableAllRules`.

#### `includeRules`

- Type: `String[]`
- Default: `[]`

Rules to include in addition to the recommended ones chosen by `enableAllRules` set to `false`.

#### `failOnError`

- Type: `Boolean`
- Default: `false`

Will cause the module build to fail if there are any errors, if option is set to `true`.

#### `quiet`

- Type: `Boolean`
- Default: `false`

Emit nothing even if there were errors happened.

## `denolint` cli

### usage

`npx denolint`

### `--config`, `-c`

Config path relative to the lint path. Config file must be a JSON file:

Example:

```json
{
  "rules": {
    "tags": ["recommended"],
    "exclude": [
      "no-explicit-any",
      "ban-unknown-rule-code",
      "no-window-prefix",
      "no-empty-interface",
      "ban-types",
      "ban-untagged-todo",
      "no-unused-vars",
      "ban-ts-comment",
      "no-case-declarations",
      "no-this-alias"
    ]
  }
}
```

Checkout [deno_lint rules](https://github.com/denoland/deno_lint/tree/main/docs/rules) for all rules.
