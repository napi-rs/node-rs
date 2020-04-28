extern crate napi_build;

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const CASTAGNOLI_POLY: u32 = 0x82f63b78;

fn main() -> Result<()> {
  napi_build::setup();

  let out_dir = match env::var_os("OUT_DIR") {
    None => return Err(From::from("OUT_DIR environment variable not defined")),
    Some(out_dir) => PathBuf::from(out_dir),
  };
  write_crc_tables(&out_dir)?;

  Ok(())
}

fn write_crc_tables(out_dir: &Path) -> Result<()> {
  let out_path = out_dir.join("crc32_table.rs");
  let mut out = io::BufWriter::new(File::create(out_path)?);

  let table = make_table(CASTAGNOLI_POLY);
  let table16 = make_table16(CASTAGNOLI_POLY);

  writeln!(out, "pub const TABLE: [u32; 256] = [")?;
  for &x in table.iter() {
    writeln!(out, "    {},", x)?;
  }
  writeln!(out, "];\n")?;

  writeln!(out, "pub const TABLE16: [[u32; 256]; 16] = [")?;
  for table in table16.iter() {
    writeln!(out, "    [")?;
    for &x in table.iter() {
      writeln!(out, "        {},", x)?;
    }
    writeln!(out, "    ],")?;
  }
  writeln!(out, "];")?;

  out.flush()?;

  Ok(())
}

fn make_table16(poly: u32) -> [[u32; 256]; 16] {
  let mut tab = [[0; 256]; 16];
  tab[0] = make_table(poly);
  for i in 0..256 {
    let mut crc = tab[0][i];
    for j in 1..16 {
      crc = (crc >> 8) ^ tab[0][crc as u8 as usize];
      tab[j][i] = crc;
    }
  }
  tab
}

fn make_table(poly: u32) -> [u32; 256] {
  let mut tab = [0; 256];
  for i in 0u32..256u32 {
    let mut crc = i;
    for _ in 0..8 {
      if crc & 1 == 1 {
        crc = (crc >> 1) ^ poly;
      } else {
        crc >>= 1;
      }
    }
    tab[i as usize] = crc;
  }
  tab
}
