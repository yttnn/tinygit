use std::{fs::{File, self}, io::{Write, BufReader, BufRead}};

use crypto::{sha1::Sha1, digest::Digest};
use flate2::{Compression, write::ZlibEncoder};

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
  let mut hasher = Sha1::new();
  hasher.input_str(&blob);
  let hex = hasher.result_str();
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
    // zlib
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    match e.write_all(&blob.as_bytes()) {
      Ok(_) => {},
      Err(e) => { return Err(e.to_string()); }
    }
    let compressed = match e.finish() {
      Ok(compressed) => compressed,
      Err(e) => { return Err(e.to_string()); }
    };

    let obj_path = "./.tinygit/objects";
    let blob_path = format!("{}/{}", obj_path, index.hash);
    let mut obj_file = File::create(blob_path).unwrap();
    obj_file.write_all(&compressed).unwrap();
    
  }
  Ok(())
}

fn get_index_contents() -> Result<Vec<IndexContent>, String> {
  let index_file = match File::open("./.tinygit/index") {
    Ok(file) => file,
    Err(_) => { return Err("index file not exists".to_string()); }
  };

  let mut contents_list: Vec<IndexContent> = Vec::new();
  let bufreader = BufReader::new(index_file);
  for line in bufreader.lines() {
    let line = line.unwrap();
    let line_split: Vec<&str> = line.split(" ").collect();
    let content = IndexContent::new(line_split[0], line_split[1], line_split[2]);
    contents_list.push(content);
  }
  Ok(contents_list)
}

struct IndexContent {
  status: String,
  hash: String,
  path: String,
}

impl IndexContent {
  fn new(status: &str, hash: &str, path: &str) -> Self {
    Self {
      status: status.to_string(),
      hash: hash.to_string(),
      path: path.to_string(),
    }
  }
}

// #[test]
// fn test() {
//   use crate::init;

//   init::run_init().expect("init failed");
//   assert!(run_add(&"text.txt".to_string()).is_ok());
//   fs::remove_dir_all("./.tinygit").expect("remove faild");
// }