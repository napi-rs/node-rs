#!/usr/bin/env node

const { binding } = require('@node-rs/deno-lint')

const enableAllRules = process.argv.includes('--all') || process.argv.includes('-a')

const hasError = binding.denolint(__dirname, enableAllRules)

if (hasError) {
  process.exit(1)
}
