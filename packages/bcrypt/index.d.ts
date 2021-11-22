/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export const DEFAULT_COST: number
export function genSaltSync(round: number, version: string): string
export function genSalt(round: number, version: string, signal?: AbortSignal | undefined | null): Promise<string>
export function hashSync(input: string | Buffer, cost?: number | undefined | null): string
export function hash(
  input: string | Buffer,
  cost?: number | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<string>
export function verifySync(input: string | Buffer, hash: string | Buffer): boolean
export function verify(
  password: string | Buffer,
  hash: string | Buffer,
  signal?: AbortSignal | undefined | null,
): Promise<boolean>
