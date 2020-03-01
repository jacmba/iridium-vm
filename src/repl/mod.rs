use std;
use std::io;
use std::io::Write;
use super::vm::VM;

pub struct REPL{
  command_buffer:  Vec<String>,
  vm: VM
}

impl REPL {
  pub fn new() -> REPL {
    REPL {
      vm: VM::new(),
      command_buffer: vec![]
    }
  }

  pub fn run(&mut self) {
    println!("Welcome to the Iridium REPL!");
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        print!(">>> ");
        io::stdout().flush().expect("Unable to flush STDOUT!!");

        stdin.read_line(&mut buffer).expect("Unable to read line from user");
        let buffer = buffer.trim();
        self.command_buffer.push(String::from(buffer));
        match buffer {
          ".quit" => {
            println!("Bye! Have a nice day!");
            std::process::exit(0);
          },
          ".history" => {
            for command in &self.command_buffer {
              println!("{}", command);
            }
          },
          _ => {
            println!("Invalid input");
          }
        }
    }
  }
}
