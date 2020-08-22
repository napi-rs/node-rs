# `@node-rs/deno-lint`

![](https://github.com/napi-rs/node-rs/workflows/CI/badge.svg)
![](https://img.shields.io/npm/dm/@node-rs/deno-lint.svg?sanitize=true)

> deno_lint nodejs binding

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

lint(filepath, source, enableAllRules)
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

### `--all`, `-a`

Enable all rules flag, if not present, denolint will run with recommend rules.