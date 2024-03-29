use std::env;

use tinygit::{add, catfile::run_catfile, commit, init};

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
    eprintln!("args not found");
    return;
  }

  if args[1] == "init" {
    match init::run_init() {
      Ok(_) => { println!("tinygit init"); }
      Err(e) => { eprintln!("{}", e); }
    }
    return;
  }
  if args[1] == "add" {
    if args.len() == 2 {
      eprintln!("nothing specified, nothing added");
      return;
    }
    if args.len() >= 4 {
      eprintln!("too many args to \"add\"");
      return;
    }

    match add::run_add(&args[2]) {
      Ok(_) => { println!("tinygit add"); }
      Err(e) => { eprintln!("{}", e); }
    }
    
    return;
  }
  if args[1] == "commit" {
    if args.len() != 3 {
      eprintln!("no commit message");
      return;
    }
    match commit::run_commit(&args[2]) {
      Ok(_) => { println!("tinigit commit"); }
      Err(e) => { eprintln!("{}", e); }
    }
    return;
  }
  if args[1] == "cat-file" {
    if args.len() != 3 {
      eprintln!("no hash");
      return;
    }
    match run_catfile(&args[2]) {
      Ok(_) => {},
      Err(e) => { eprintln!("{}", e); }
    }
    return;
  }
  //
  eprintln!("command not found");
}
