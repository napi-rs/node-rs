export const DEFAULT_COST: 12

export function hashSync(password: string | Buffer, round?: number): string
export function hash(password: string | Buffer, round?: number): Promise<string>
export function verifySync(password: string | Buffer, hash: string | Buffer): boolean
export function verify(password: string | Buffer, hash: string | Buffer): Promise<boolean>
