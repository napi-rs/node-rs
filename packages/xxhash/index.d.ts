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
}

export class Xxh64 {
  constructor(seed?: BigInt)
  update(input: BufferLike): this
  digest(): BigInt
}
