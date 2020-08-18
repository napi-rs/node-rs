const { readFileSync } = require('fs')

const { lint } = require('./index')

const filepath = require.resolve('rxjs/src/internal/Subscriber.ts')

const sourceCodeBuffer = readFileSync(filepath)

const result = lint(filepath, sourceCodeBuffer)

console.assert(Array.isArray(result))
console.info('deno-lint simple test passed')
