/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function xxh32(input: string | Buffer, seed?: number | undefined | null): number
export function xxh64(input: string | Buffer, seed?: BigInt | undefined | null): BigInt
export class Xxh32 {
  constructor(seed?: number | undefined | null)
  update(input: string | Buffer): this
  digest(): number
  reset(newState?: number | undefined | null): void
}
export class Xxh64 {
  constructor(seed?: BigInt | undefined | null)
  update(input: string | Buffer): this
  digest(): BigInt
  reset(newState?: BigInt | undefined | null): void
}
export namespace xxh3 {
  export function xxh64(input: string | Buffer, seed?: BigInt | undefined | null): BigInt
  export function xxh64WithSecret(input: string | Buffer, secret: Buffer): BigInt
  export function xxh128(input: string | Buffer, seed?: BigInt | undefined | null): BigInt
  export function xxh128WithSecret(input: string | Buffer, secret: Buffer): BigInt
  export class Xxh3 {
    static withSeed(seed?: BigInt | undefined | null): Xxh3
    static withSecret(secret: Buffer): Xxh3
    update(input: string | Buffer): this
    digest(): BigInt
    reset(): void
  }
}
