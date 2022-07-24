use nix::unistd::{fork, ForkResult};
use std::process;

fn main() {
    println!("hello, world! (pid: {})\n", process::id());
    match fork() {
        Err(_) => println!("fork failed."),
        Ok(ForkResult::Child) => {
            println!("hello, I am child. (pid: {})", process::id())
        },
        Ok(ForkResult::Parent { child, .. }) => {
            println!("hello, I am parent of {} (pid: {})", child, process::id())
        },
    }
}
