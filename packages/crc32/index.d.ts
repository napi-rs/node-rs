/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function crc32c(input: string | Buffer, initialState?: number | undefined | null): number
export function crc32(inputData: string | Buffer, initialState?: number | undefined | null): number
