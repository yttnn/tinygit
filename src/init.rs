use std::fs::{self, File};

pub fn run_init() -> Result<(), String> {
  match fs::create_dir("./.tinygit") {
    Ok(_) => {}
    Err(_) => { return Err("faild to create dir \".tinygit\"".to_string()); }
  }

  match File::create("./.tinygit/index") {
    Ok(_) => {}
    Err(_) => { return Err("faild to create file \".tinygit/index\"".to_string()); }
  }

  match fs::create_dir("./.tinygit/objects") {
    Ok(_) => {}
    Err(_) => { return Err("faild to create dir \".tinygit\"".to_string()); }
  }

  match fs::create_dir("./.tinygit/refs") {
    Ok(_) => {}
    Err(_) => { return Err("faild to create dir \".tinygit/refs\"".to_string()); }
  }

  match fs::create_dir("./.tinygit/refs/heads") {
    Ok(_) => {}
    Err(_) => { return Err("faild to create dir \".tinygit/refs/heads\"".to_string()); }
  }

  match File::create("./.tinygit/refs/heads/main") {
    Ok(_) => {}
    Err(_) => { return Err("faild to crate file \".tinygit/refs/heads/main".to_string()); }
  }

  Ok(())
}

#[test]
fn test() {
  assert!(run_init().is_ok());
  fs::remove_dir_all("./.tinygit").expect("remove faild");
}