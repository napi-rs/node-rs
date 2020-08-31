#!/usr/bin/env node

const { binding } = require('@node-rs/deno-lint')

const { argv } = process

const enableAllRules = argv.includes('--all') || argv.includes('-a')

const hasError = binding.denolint(__dirname, enableAllRules)

if (hasError) {
  process.exit(1)
}
