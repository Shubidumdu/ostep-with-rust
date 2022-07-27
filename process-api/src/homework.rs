use nix::unistd::{fork, ForkResult};
use std::process;

pub fn h1() {
  let mut x = 100;
  println!("The value is {}", x);
  match fork() {
    Err(_) => {
        println!("fork failed.");
        process::exit(1);
    }
    Ok(ForkResult::Child) => {
        println!("hello, I am child. (pid: {})", process::id())
    },
    Ok(ForkResult::Parent { child }) => {
        println!("hello, I am parent of {} (pid: {})", child, process::id())
    },
}
}