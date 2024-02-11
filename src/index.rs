use std::{fs::File, io::{BufRead, BufReader}};



pub fn get_index_contents() -> Result<Vec<IndexContent>, String> {
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

pub struct IndexContent {
  pub status: String,
  pub hash: String,
  pub path: String,
}

impl IndexContent {
  pub fn new(status: &str, hash: &str, path: &str) -> Self {
    Self {
      status: status.to_string(),
      hash: hash.to_string(),
      path: path.to_string(),
    }
  }
}