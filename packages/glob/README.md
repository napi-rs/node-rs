# `@napi-rs/glob`

Rust's [`glob` crate](https://docs.rs/glob/latest/glob/) bindings for Node.js

## Install

```
yarn add @napi-rs/glob
```

## Usage

```ts
const path = require('path')
const { glob } = require('@napi-rs/glob')

const allJsFiles = glob(path.join(__dirname, '**/*.js'))
```

### Options

[Rust glob's `MatchOptions`](https://docs.rs/glob/latest/glob/struct.MatchOptions.html) is accepted as second argument.

Note: keys are in camelCase.

```ts
const allFiles = glob('*', { caseSensitive: true })
```
