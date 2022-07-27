use nix::unistd::{fork, execvp, ForkResult};
use nix::sys::wait::{wait};
use std::process;
use std::io::{self, Write};

pub fn p1() {
  println!("hello, world! (pid: {})\n", process::id());
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

pub fn p2() {
  println!("hello, world! (pid: {})\n", process::id());
  match fork() {
      Err(_) => {
          println!("fork failed.");
          process::exit(1);
      }
      Ok(ForkResult::Child) => {
          println!("hello, I am child. (pid: {})", process::id())
      },
      Ok(ForkResult::Parent { child }) => {
          match wait() {
              Err(_) => {
                  println!("wait failed.");
                  process::exit(1);
              },
              Ok(_) => {
                  println!("hello, I am parent of {} (pid: {})", child, process::id())
              },
          }
      },
  }
}

pub fn p3() {
    println!("hello, world! (pid: {})\n", process::id());
    match fork() {
        Err(_) => {
            println!("fork failed.");
            process::exit(1);
        }
        Ok(ForkResult::Child) => {
            println!("hello, I am child. (pid: {})", process::id());
            let output = process::Command::new("echo")
                .args(["yeaaahhhhhh!!!"])
                .output()
                .expect("failed to execute process");
            println!("status: {}", output.status);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        },
        Ok(ForkResult::Parent { child }) => {
            match wait() {
                Err(_) => {
                    println!("wait failed.");
                    process::exit(1);
                },
                Ok(_) => {
                    println!("hello, I am parent of {} (pid: {})", child, process::id())
                },
            }
        },
    }
}
