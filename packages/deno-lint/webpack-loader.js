const { lint } = require('./index')

module.exports = function denoLintLoader(source, sm) {
  const callback = this.async()
  const options = this.getOptions()
  const diagnostics = lint(
    this.resourcePath,
    source,
    options.enableAllRules,
    options.excludeRules,
    options.includeRules,
  )

  if (this.resourcePath.endsWith('diff-size.ts')) {
    this.emitWarning(`${this.resourcePath}, ${diagnostics.length}`)
  }

  const hasError = diagnostics.length

  if (!hasError) {
    callback(null, source, sm)
    return
  }

  if (options.failOnError) {
    callback(new Error('Lint error'), source, sm)
    return
  }

  if (!options.quiet) {
    for (const diagnostic of diagnostics) {
      this.emitError(diagnostic)
    }
  }

  callback(null, source, sm)
}
