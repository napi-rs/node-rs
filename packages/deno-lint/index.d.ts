/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function lint(
  fileName: string,
  sourceCode: string | Buffer,
  allRules?: boolean | undefined | null,
): Array<string>
export function denolint(dirname: string, configPath: string): boolean
