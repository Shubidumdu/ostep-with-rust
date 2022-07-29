use nix::unistd::{fork, ForkResult};
use std::process;

pub fn h1() {
  println!("---------------------------------");
  let mut x = 100;
  println!("The value is {}", x);
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello, I am child. And the value is {}. (pid: {})", x, process::id());
      x = 120;
      println!("hello, I am child. And the value is {}. (pid: {})", x, process::id());
    },
    Ok(ForkResult::Parent { child }) => {
      println!("hello, I am parent of {}. And the value is {}. (pid: {})", child, x, process::id());
      x = 130;
      println!("hello, I am parent of {}. And the value is {}. (pid: {})", child, x, process::id());
    },
  }
}
