use crate::bytes;
use crate::crc32_table::{TABLE, TABLE16};

#[inline]
#[cfg(not(target_arch = "x86_64"))]
pub fn crc32c(buf: &[u8]) -> u32 {
  crc32c_slice16(buf)
}

/// Returns the CRC32 checksum of `buf` using the Castagnoli polynomial.
#[inline]
#[cfg(target_arch = "x86_64")]
pub fn crc32c(buf: &[u8]) -> u32 {
  if is_x86_feature_detected!("sse4.2") {
    // SAFETY: When sse42 is true, we are guaranteed to be running on
    // a CPU that supports SSE 4.2.
    unsafe { crc32c_sse(buf) }
  } else {
    crc32c_slice16(buf)
  }
}

#[inline]
#[cfg(not(target_arch = "x86_64"))]
pub fn crc32c_append(buf: &[u8], crc: u32) -> u32 {
  append_crc32c_slice16(buf, crc)
}

/// Returns the CRC32 checksum of `buf` using the Castagnoli polynomial.
#[inline]
#[cfg(target_arch = "x86_64")]
pub fn crc32c_append(buf: &[u8], crc: u32) -> u32 {
  if is_x86_feature_detected!("sse4.2") {
    // SAFETY: When sse42 is true, we are guaranteed to be running on
    // a CPU that supports SSE 4.2.
    unsafe { append_crc32c_sse(buf, crc) }
  } else {
    append_crc32c_slice16(buf, crc)
  }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.2")]
unsafe fn crc32c_sse(buf: &[u8]) -> u32 {
  append_crc32c_sse(buf, 0)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse4.2")]
unsafe fn append_crc32c_sse(buf: &[u8], initial_crc: u32) -> u32 {
  use std::arch::x86_64::*;
  let mut crc = !initial_crc;
  // SAFETY: This is safe since alignment is handled by align_to (oh how I
  // love you) and since 8 adjacent u8's are guaranteed to have the same
  // in-memory representation as u64 for all possible values.
  let (prefix, u64s, suffix) = buf.align_to::<u64>();
  for &b in prefix {
    // SAFETY: Safe since we have sse4.2 enabled.
    crc = _mm_crc32_u8(crc, b);
  }
  for &n in u64s {
    // SAFETY: Safe since we have sse4.2 enabled.
    crc = _mm_crc32_u64(crc as u64, n) as u32;
  }
  for &b in suffix {
    // SAFETY: Safe since we have sse4.2 enabled.
    crc = _mm_crc32_u8(crc, b);
  }
  !crc
}

/// Returns the CRC32 checksum of `buf` using the Castagnoli polynomial.
#[inline]
fn crc32c_slice16(buf: &[u8]) -> u32 {
  append_crc32c_slice16(buf, 0)
}

#[inline]
fn append_crc32c_slice16(mut buf: &[u8], initial_crc: u32) -> u32 {
  let mut crc = !initial_crc;
  while buf.len() >= 16 {
    crc ^= bytes::read_u32_le(buf);
    crc = TABLE16[0][buf[15] as usize]
      ^ TABLE16[1][buf[14] as usize]
      ^ TABLE16[2][buf[13] as usize]
      ^ TABLE16[3][buf[12] as usize]
      ^ TABLE16[4][buf[11] as usize]
      ^ TABLE16[5][buf[10] as usize]
      ^ TABLE16[6][buf[9] as usize]
      ^ TABLE16[7][buf[8] as usize]
      ^ TABLE16[8][buf[7] as usize]
      ^ TABLE16[9][buf[6] as usize]
      ^ TABLE16[10][buf[5] as usize]
      ^ TABLE16[11][buf[4] as usize]
      ^ TABLE16[12][(crc >> 24) as u8 as usize]
      ^ TABLE16[13][(crc >> 16) as u8 as usize]
      ^ TABLE16[14][(crc >> 8) as u8 as usize]
      ^ TABLE16[15][(crc) as u8 as usize];
    buf = &buf[16..];
  }
  for &b in buf {
    crc = TABLE[((crc as u8) ^ b) as usize] ^ (crc >> 8);
  }
  !crc
}
