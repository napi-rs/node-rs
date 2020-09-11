/**
 * @param {string | Buffer} input string or buffer to calculate
 * @param {number} crc u32 number, default 0
 * @returns {number} u32 number
 */
export function crc32(input: Buffer | string, crc?: number): number
/**
 * @param {string | Buffer} input string or buffer to calculate
 * @param {number} crc u32 number, default 0
 * @returns {number} u32 number
 */
export function crc32c(input: Buffer | string, crc?: number): number
