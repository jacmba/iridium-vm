use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
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
          ".program" => {
            println!("Listing instructions in current VM's program vector:");
            for instruction in self.vm.get_program() {
              println!("{:?}", instruction);
            }
            println!("End of program listing");
          },
          ".registers" => {
            println!("Listing contents of VM registers");
            println!("{:?}", self.vm.get_registers());
            println!("End of registers listing");
          },
          ".dump" => {
            println!("---- Printing VM dump ----");
            print!("{:?}", self.vm);
            println!("---- End printing VM dump");
          }
          _ => {
            let results = self.parse_hex(buffer);
            match results {
                Ok(bytes) => {
                  for byte in bytes {
                    self.vm.add_byte(byte);
                  }
                },
                Err(_e) => {
                  println!("Invalid HEX string")
                },
            }
            
            self.vm.run_once();
          }
        }
    }
  }

  fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
    let split = i.split(" ").collect::<Vec<&str>>();
    let mut results: Vec<u8> = vec![];

    for hex_string in split {
      let byte = u8::from_str_radix(&hex_string, 16);
      match byte {
        Ok(result) => {
          results.push(result);
        },
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok(results)
  }
}
