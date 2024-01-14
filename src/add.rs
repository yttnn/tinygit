use std::{fs::{File, self}, io::Write};

use crypto::{sha1::Sha1, digest::Digest};

pub fn run_add(file_path: &String) -> Result<(), String> {
  write_index(file_path)?;
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
  let mut hasher = Sha1::new();
  hasher.input_str(&blob);
  let hex = hasher.result_str();
  let status = "100644";
  println!("{}", hex);
  index_file.write(&format!("{} {} {}\n", status, hex, file_path).as_bytes()).unwrap();

  println!("{}", file_text);
  Ok(())
}

// #[test]
// fn test() {
//   use crate::init;

//   init::run_init().expect("init failed");
//   assert!(run_add(&"text.txt".to_string()).is_ok());
//   fs::remove_dir_all("./.tinygit").expect("remove faild");
// }