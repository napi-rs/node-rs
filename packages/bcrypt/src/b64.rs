use crate::errors::{BcryptError, BcryptResult};
use phf::phf_map;
use radix64::STD;

// Decoding table from bcrypt base64 to standard base64 and standard -> bcrypt
// Bcrypt has its own base64 alphabet
// ./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
static BCRYPT_TO_STANDARD: phf::Map<char, &'static str> = phf_map! {
  '/' => "B",
  '.' => "A",
  '1' => "3",
  '0' => "2",
  '3' => "5",
  '2' => "4",
  '5' => "7",
  '4' => "6",
  '7' => "9",
  '6' => "8",
  '9' => "/",
  '8' => "+",
  'A' => "C",
  'C' => "E",
  'B' => "D",
  'E' => "G",
  'D' => "F",
  'G' => "I",
  'F' => "H",
  'I' => "K",
  'H' => "J",
  'K' => "M",
  'J' => "L",
  'M' => "O",
  'L' => "N",
  'O' => "Q",
  'N' => "P",
  'Q' => "S",
  'P' => "R",
  'S' => "U",
  'R' => "T",
  'U' => "W",
  'T' => "V",
  'W' => "Y",
  'V' => "X",
  'Y' => "a",
  'X' => "Z",
  'Z' => "b",
  'a' => "c",
  'c' => "e",
  'b' => "d",
  'e' => "g",
  'd' => "f",
  'g' => "i",
  'f' => "h",
  'i' => "k",
  'h' => "j",
  'k' => "m",
  'j' => "l",
  'm' => "o",
  'l' => "n",
  'o' => "q",
  'n' => "p",
  'q' => "s",
  'p' => "r",
  's' => "u",
  'r' => "t",
  'u' => "w",
  't' => "v",
  'w' => "y",
  'v' => "x",
  'y' => "0",
  'x' => "z",
  'z' => "1",
};

static STANDARD_TO_BCRYPT: phf::Map<char, &'static str> = phf_map! {
  'B' => "/",
  'A' => ".",
  '3' => "1",
  '2' => "0",
  '5' => "3",
  '4' => "2",
  '7' => "5",
  '6' => "4",
  '9' => "7",
  '8' => "6",
  '/' => "9",
  '+' => "8",
  'C' => "A",
  'E' => "C",
  'D' => "B",
  'G' => "E",
  'F' => "D",
  'I' => "G",
  'H' => "F",
  'K' => "I",
  'J' => "H",
  'M' => "K",
  'L' => "J",
  'O' => "M",
  'N' => "L",
  'Q' => "O",
  'P' => "N",
  'S' => "Q",
  'R' => "P",
  'U' => "S",
  'T' => "R",
  'W' => "U",
  'V' => "T",
  'Y' => "W",
  'X' => "V",
  'a' => "Y",
  'Z' => "X",
  'b' => "Z",
  'c' => "a",
  'e' => "c",
  'd' => "b",
  'g' => "e",
  'f' => "d",
  'i' => "g",
  'h' => "f",
  'k' => "i",
  'j' => "h",
  'm' => "k",
  'l' => "j",
  'o' => "m",
  'n' => "l",
  'q' => "o",
  'p' => "n",
  's' => "q",
  'r' => "p",
  'u' => "s",
  't' => "r",
  'w' => "u",
  'v' => "t",
  'y' => "w",
  'x' => "v",
  '0' => "y",
  'z' => "x",
  '1' => "z",
  '=' => "=",
};

/// First encode to base64 standard and then replaces char with the bcrypt
/// alphabet and removes the '=' chars
pub fn encode(words: &[u8]) -> String {
  let hash = STD.encode(words);
  let mut res = String::with_capacity(hash.len());

  for ch in hash.chars() {
    // can't fail
    let replacement = STANDARD_TO_BCRYPT.get(&ch).unwrap();
    if replacement != &"=" {
      res.push_str(replacement);
    }
  }

  res
}

// Can potentially panic if the hash given contains invalid characters
pub fn decode(hash: &str) -> BcryptResult<Vec<u8>> {
  let mut res = String::with_capacity(hash.len());
  for ch in hash.chars() {
    if let Some(c) = BCRYPT_TO_STANDARD.get(&ch) {
      res.push_str(c);
    } else {
      return Err(BcryptError::DecodeError(ch, hash.to_string()));
    }
  }

  // Bcrypt base64 has no padding but standard has
  // so we need to actually add padding ourselves
  if hash.len() % 4 > 0 {
    let padding = 4 - hash.len() % 4;
    for _ in 0..padding {
      res.push_str("=");
    }
  }

  // safe unwrap: if we had non standard chars, it would have errored before
  Ok(STD.decode(&res).unwrap())
}

#[cfg(test)]
mod tests {
  use super::{decode, encode};

  #[test]
  fn can_decode_bcrypt_base64() {
    let hash = "YETqZE6eb07wZEO";
    assert_eq!(
      "hello world",
      String::from_utf8_lossy(&decode(hash).unwrap())
    );
  }

  #[test]
  fn can_encode_to_bcrypt_base64() {
    let expected = "YETqZE6eb07wZEO";
    assert_eq!(encode("hello world".as_bytes()), expected);
  }

  #[test]
  fn decode_errors_with_unknown_char() {
    assert!(decode("YETqZE6e_b07wZEO").is_err());
  }
}
