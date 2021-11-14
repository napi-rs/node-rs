export type BufferLike =
  | Buffer
  | string
  | Uint8Array
  | ArrayBuffer
  | SharedArrayBuffer
  | ReadonlyArray<number>
  | number[]

export function xxh32(input: BufferLike, seed?: number): number
export function xxh64(input: BufferLike, seed?: BigInt): BigInt

export class Xxh32 {
  constructor(seed?: number)
  update(input: BufferLike): this
  digest(): number
  reset(): void
}

export class Xxh64 {
  constructor(seed?: BigInt)
  update(input: BufferLike): this
  digest(): BigInt
  reset(): void
}

export class Xxh3 {
  static withSeed(seed?: BigInt): Xxh3
  static withSecret(secret: BufferLike): Xxh3
  private constructor()
  update(input: BufferLike): this
  digest(): BigInt
  reset(): void
}

export const xxh3: {
  xxh64: (input: BufferLike, seed?: BigInt) => BigInt
  xxh64WithSecret: (input: BufferLike, secret: BufferLike) => BigInt
  xxh128: (input: BufferLike, seed?: BigInt) => BigInt
  xxh128WithSecret: (input: BufferLike, secret: BufferLike) => BigInt
  Xxh3: typeof Xxh3
}
