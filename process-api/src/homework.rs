use nix::sys::wait::{wait, waitpid};
use nix::unistd::{close, pipe, read, write};
use nix::unistd::{fork, ForkResult};
use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::io::prelude::*;

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
      println!(
        "hello, I am child. And the value is {}. (pid: {})",
        x,
        process::id()
      );
      x = 120;
      println!(
        "hello, I am child. And the value is {}. (pid: {})",
        x,
        process::id()
      );
    }
    Ok(ForkResult::Parent { child }) => {
      println!(
        "hello, I am parent of {}. And the value is {}. (pid: {})",
        child,
        x,
        process::id()
      );
      x = 130;
      println!(
        "hello, I am parent of {}. And the value is {}. (pid: {})",
        child,
        x,
        process::id()
      );
    }
  }
}

pub fn h2() -> std::io::Result<()> {
  println!("---------------------------------");
  let mut file = File::create("./foo.txt")?;
  file.write_all(b"Hello, world!\n")?;
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!(
        "hello, I am child. And the file is {:?}. (pid: {})",
        file,
        process::id()
      );
      file.write_all(b"Hello, i'm from child!\n")?;
      println!(
        "hello, I am child. And the file is {:?}. (pid: {})",
        file,
        process::id()
      );
    }
    Ok(ForkResult::Parent { child }) => {
      println!(
        "hello, I am parent of {}. And the file is {:?}. (pid: {})",
        child,
        file,
        process::id()
      );
      file.write_all(b"Hello, i'm from parent!\n")?;
      println!(
        "hello, I am parent of {}. And the file is {:?}. (pid: {})",
        child,
        file,
        process::id()
      );
    }
  }
  Ok(())
}

pub fn h3() {
  println!("---------------------------------");
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello.");
    }
    Ok(ForkResult::Parent { child: _ }) => match wait() {
      Err(_) => {
        println!("wait failed.");
        process::exit(1);
      }
      Ok(_) => {
        println!("goodbye.");
      }
    },
  }
}

pub fn h4() {
  println!("---------------------------------");
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello, I am child. (pid: {})", process::id());
      let output = process::Command::new("ls")
        .args(["./"])
        .output()
        .expect("failed to execute process");
      println!("status: {}", output.status);
      io::stdout().write_all(&output.stdout).unwrap();
      io::stderr().write_all(&output.stderr).unwrap();
    }
    Ok(ForkResult::Parent { child }) => match wait() {
      Err(_) => {
        println!("wait failed.");
        process::exit(1);
      }
      Ok(_) => {
        println!("hello, I am parent of {} (pid: {})", child, process::id())
      }
    },
  }
}

pub fn h5() {
  println!("---------------------------------");
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello, I am child. (pid: {})", process::id());
      match wait() {
        Err(error) => {
          println!("wait failed. : {:?}", error);
          process::exit(1);
        }
        Ok(result) => {
          println!("The wait result is : {:?}", result);
        }
      };
    }
    Ok(ForkResult::Parent { child }) => match wait() {
      Err(_) => {
        println!("wait failed.");
        process::exit(1);
      }
      Ok(result) => {
        println!("The wait result is : {:?}", result);
        println!("hello, I am parent of {} (pid: {})", child, process::id())
      }
    },
  }
}

pub fn h6() {
  println!("---------------------------------");
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello, I am child. (pid: {})", process::id());
    }
    Ok(ForkResult::Parent { child }) => match waitpid(child, Option::None) {
      Err(_) => {
        println!("wait failed.");
        process::exit(1);
      }
      Ok(result) => {
        println!("The wait result is : {:?}", result);
        println!("hello, I am parent of {} (pid: {})", child, process::id())
      }
    },
  }
}

pub fn h7() -> std::io::Result<()> {
  println!("---------------------------------");
  let mut file = File::create("./foo.txt")?;
  file.write_all(b"Hello, world!\n")?;
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello, I am child. (pid: {})", process::id());
      drop(file);
    }
    Ok(ForkResult::Parent { child }) => match wait() {
      Err(_) => {
        println!("wait failed.");
        process::exit(1);
      }
      Ok(result) => {
        println!("The wait result is : {:?}", result);
        println!("hello, I am parent of {} (pid: {})", child, process::id());
        file.write_all(b"Hello from parent!")?;
      }
    },
  }
  Ok(())
}

pub fn h8() {
  println!("---------------------------------");
  let (rd, wr) = pipe().unwrap();
  match fork() {
    Err(_) => {
      println!("fork failed.");
      process::exit(1);
    }
    Ok(ForkResult::Child) => {
      println!("hello, I am child. (pid: {})", process::id());
      match write(wr, b"Hello from first child!") {
        Ok(_) => {
          println!("write succeed.");
        },
        Err(_) => {
          println!("write failed.");
        }
      }
    }
    Ok(ForkResult::Parent { child }) => match wait() {
      Err(_) => {
        println!("wait failed.");
        process::exit(1);
      }
      Ok(_) => {
        println!("hello, I am parent of {} (pid: {})", child, process::id());
        let mut buf = [0u8; 1024];
        match read(rd, &mut buf) {
          Ok(size) => {
            println!("read: {:?}", String::from_utf8((&buf[0..size]).to_vec()).unwrap());
            close(rd).unwrap();
            close(wr).unwrap();
          },
          Err(_) => {
            println!("read failed.");
          }
        }
      }
    },
  }
}
