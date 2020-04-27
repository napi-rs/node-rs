use std::convert::TryInto;

/// Read a u32 in little endian format from the beginning of the given slice.
/// This panics if the slice has length less than 4.
#[inline]
pub fn read_u32_le(slice: &[u8]) -> u32 {
  u32::from_le_bytes(slice[..4].try_into().unwrap())
}
