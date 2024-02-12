use std::{fs::File, io::Read, path::Path};

use crate::zlib;

pub fn run_catfile(hash: &String) -> Result<(), String> {
  let obj_path = format!("./.tinygit/objects/{}", hash);
  let path = Path::new(&obj_path);
  match path.try_exists() {
    Ok(_) => {},
    Err(_) => { return Err("object file not found".to_string()); }
  }

  match File::open(obj_path) {
    Err(e) => { return Err(format!("object file open error : {}", e.to_string())); },
    Ok(mut file) => {
      let mut buffer = Vec::new();
      let _ = file.read_to_end(&mut buffer);
      let decode = zlib::decode(buffer);
      // if decode.is_err() { return Err("decode error".to_string()); }
      match decode {
        Err(e) => { return Err(e.to_string()); }
        Ok(_) => {}
      }
      let decode = decode.unwrap();
      println!("{}", decode);
    }
  }
  Ok(())
}