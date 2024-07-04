#!/usr/bin/env node

const { cli } = require('./cli')

cli
  .run(process.argv.slice(2), {
    stdin: process.stdin,
    stdout: process.stdout,
    stderr: process.stderr,
  })
  .then((code) => {
    if (code !== 0) {
      process.exit(code)
    }
  })
  .catch((e) => {
    console.error(e)
    process.exit(1)
  })
