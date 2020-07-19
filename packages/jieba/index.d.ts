export function load(): void
export function loadDict(dict: Buffer): void
export function cut(sentence: string | Buffer, hmm?: boolean): string[]
export function cutAll(sentence: string | Buffer): string[]

export interface TagResult {
  word: string
  tag: string
}

export interface Keyword {
  keyword: string
  weight: number
}

export function tag(sentence: string | Buffer, hmm?: boolean): TagResult[]

export function extract(sentence: string | Buffer, topn: number, allowedPos?: string[]): Keyword[]
