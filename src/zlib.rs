use std::io::{Error, Write};

use flate2::{write::ZlibEncoder, Compression};

pub fn encode(s: &str) -> Result<Vec<u8>, Error> {
  let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
  e.write_all(s.as_bytes())?;
  e.finish()
}