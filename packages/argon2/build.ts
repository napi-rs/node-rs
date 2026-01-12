import { createBuildCommand, NapiCli } from '@napi-rs/cli'

const build = createBuildCommand(process.argv.slice(2))
const options = build.getOptions()
const cli = new NapiCli()

if (options.target !== 'wasm32-wasip1-threads') {
  options.features ??= []
  options.features.push('parallel')
}

const { task } = await cli.build(options)

await task
