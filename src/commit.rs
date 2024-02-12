use std::{fs::File, io::Write};

use crate::{index::get_index_contents, sha1::generate_hash, zlib};

pub fn run_commit(message: &String) -> Result<(), String> {
  create_commit_object(message)?;
  Ok(())
}

fn create_commit_object(message: &String) -> Result<(), String> {
  let tree_hash = create_tree_object()?;
  let commit_contents = format!("tree {}\nmessage {}\n", tree_hash, message);
  let commit = format!("commit {}\0{}", commit_contents.as_bytes().len(), commit_contents);

  let hex = generate_hash(&commit);
  let compressed = match zlib::encode(&commit) {
    Ok(c) => c,
    Err(e) => { return Err(e.to_string()); }
  };

  let obj_path = "./.tinygit/objects";
  let commit_path = format!("{}/{}", obj_path, hex);
  let mut obj_file = File::create(commit_path).unwrap();
  obj_file.write_all(&compressed).unwrap();
  println!("create commit object {}", hex);

  Ok(())
}

fn create_tree_object() -> Result<String, String> { // "return Ok(hash)" is not good
  let index_list = get_index_contents()?;
  let mut index_contents = "".to_string();
  for index in index_list {
    index_contents = format!("{}blob {}\n", index_contents, index.hash);
  }
  // TODO: tree object contains "tree object"
  
  let tree = format!("tree {}\0{}", index_contents.as_bytes().len(), index_contents);
  let hex = generate_hash(&tree);
  let compressed = match zlib::encode(&tree) {
    Ok(c) => c,
    Err(e) => { return Err(e.to_string()); }
  };

  let obj_path = "./.tinygit/objects";
  let tree_path = format!("{}/{}", obj_path, hex);
  let mut obj_file = File::create(tree_path).unwrap();
  obj_file.write_all(&compressed).unwrap();
  println!("create tree object {}", hex);

  Ok(hex)
}