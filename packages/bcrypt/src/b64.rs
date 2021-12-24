use phf::phf_map;
use radix64::STD;

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

#[cfg(test)]
mod tests {
  use super::encode;

  #[test]
  fn can_encode_to_bcrypt_base64() {
    let expected = "YETqZE6eb07wZEO";
    assert_eq!(encode(b"hello world"), expected);
  }
}
