import { denolint } from '@node-rs/deno-lint'
import { Cli, Command, Option } from 'clipanion'

class LintCommand extends Command {
  static usage = {
    description: 'denolint [options] [dirs...]',
  }

  private readonly dirs: string[] = Option.Rest()

  private readonly rootDir = Option.String('-r,--root', { required: false })

  private readonly configPath = Option.String('-c,--config', { required: false })

  private readonly checkOnly = Option.Boolean('--check-only', { required: false })

  async execute() {
    const hasError = denolint(this.rootDir ?? __dirname, this.configPath ?? '.denolint.json', this.dirs)
    return Promise.resolve(hasError && !this.checkOnly ? 1 : 0)
  }
}

export const cli = new Cli({
  binaryLabel: 'deno-lint',
  binaryVersion: require('./package.json').version,
})

cli.register(LintCommand)
