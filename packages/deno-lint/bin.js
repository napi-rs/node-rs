#!/usr/bin/env node

const { binding } = require('@node-rs/deno-lint')

const hasError = binding.denolint(__dirname)

if (hasError) {
  process.exit(1)
}
