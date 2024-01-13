use std::env;

mod init;

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

  eprintln!("command not found");
}
