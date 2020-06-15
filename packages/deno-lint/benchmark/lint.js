const { readFileSync } = require('fs')

const { parseForESLint } = require('@typescript-eslint/parser')
const { Suite } = require('benchmark')
const chalk = require('chalk')
const { Linter, SourceCode } = require('eslint')

const { lint } = require('../index')

const suite = new Suite('Lint benchmark')

const filepath = require.resolve('rxjs/src/internal/Subscriber.ts')

const tsconfigPath = require.resolve('rxjs/src/tsconfig.json')

const sourceCodeBuffer = readFileSync(filepath)
const sourcecode = sourceCodeBuffer.toString('utf-8')

const linter = new Linter()

suite
  .add('@node-rs/deno-lint', () => {
    lint(filepath, sourceCodeBuffer)
  })
  .add('eslint', () => {
    const parseForESLintResult = parseForESLint(sourcecode, {
      filePath: filepath,
      sourceType: 'module',
      ecmaVersion: 2019,
      project: tsconfigPath,
      loc: true,
      range: true,
      tokens: true,
      comment: true,
    })

    const sc = new SourceCode({
      text: sourcecode,
      ...parseForESLintResult,
    })

    linter.verify(sc, {}, filepath)
  })
  .on('cycle', function (event) {
    console.info(String(event.target))
  })
  .on('complete', function () {
    console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}`)
  })
  .run()
