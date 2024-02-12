use std::io::{self, Error, Read, Write};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};

pub fn encode(s: &str) -> Result<Vec<u8>, Error> {
  let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
  e.write_all(s.as_bytes())?;
  e.finish()
}

pub fn decode(bytes: Vec<u8>) -> Result<String, io::Error> {
  let mut z = ZlibDecoder::new(&bytes[..]); // `..` means Range Full
  let mut s = String::new();
  z.read_to_string(&mut s)?;
  Ok(s)
}