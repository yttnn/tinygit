use std::{fs::{File, self}, io::Write};

use crate::{index::get_index_contents, sha1::generate_hash, zlib};

pub fn run_add(file_path: &String) -> Result<(), String> {
  write_index(file_path)?;
  create_object()?;
  Ok(())
}

fn write_index(file_path: &String) -> Result<(), String> {
    let mut index_file = match File::create("./.tinygit/index") {
    Ok(file) => file,
    Err(_) => { return Err("index file not exists".to_string()); }
  };

  let file_text = match fs::read_to_string(file_path) {
    Ok(text) => text, 
    Err(_) => { return Err(format!("{} not exists", file_path)); }
  };

  let blob = format!("blob {}\0{}", file_text.as_bytes().len(), file_text);
  let hex = generate_hash(&blob);
  let status = "100644";
  println!("{}", hex);
  index_file.write(&format!("{} {} {}\n", status, hex, file_path).as_bytes()).unwrap();

  println!("{}", file_text);
  Ok(())
}

fn create_object() -> Result<(), String> {
  let index_list = get_index_contents()?;
  for index in index_list {
    let path = index.path;
    let text = match fs::read_to_string(path) {
      Ok(text) => text,
      Err(e) => { return Err(e.to_string()); }
    };

    let blob = format!("blob {}\0{}", text.as_bytes().len(), text);
    let compressed = match zlib::encode(&blob) {
      Ok(c) => c,
      Err(e) => { return Err(e.to_string()); }
    };

    let obj_path = "./.tinygit/objects";
    let blob_path = format!("{}/{}", obj_path, index.hash);
    let mut obj_file = File::create(blob_path).unwrap();
    obj_file.write_all(&compressed).unwrap();
    
  }
  Ok(())
}

// #[test]
// fn test() {
//   use crate::init;

//   init::run_init().expect("init failed");
//   assert!(run_add(&"text.txt".to_string()).is_ok());
//   fs::remove_dir_all("./.tinygit").expect("remove faild");
// }