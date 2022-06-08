use super::assembler::program_parser::*;
use super::assembler::*;
use super::vm::VM;
use nom::types::CompleteStr;
use std;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;

pub struct REPL {
  command_buffer: Vec<String>,
  vm: VM,
  asm: Assembler,
}

impl REPL {
  pub fn new() -> REPL {
    REPL {
      vm: VM::new(),
      command_buffer: vec![],
      asm: Assembler::new(),
    }
  }

  pub fn run(&mut self) {
    println!("Welcome to the Iridium REPL!");
    loop {
      let mut buffer = String::new();
      let stdin = io::stdin();

      print!(">>> ");
      io::stdout().flush().expect("Unable to flush STDOUT!!");

      stdin
        .read_line(&mut buffer)
        .expect("Unable to read line from user");
      let buffer = buffer.trim().to_lowercase();
      let input = String::from(&buffer);
      self.command_buffer.push(buffer);
      match input.as_str() {
        ".quit" => {
          println!("Bye! Have a nice day!");
          std::process::exit(0);
        }
        ".history" => {
          for command in &self.command_buffer {
            println!("{}", command);
          }
        }
        ".program" => {
          println!("Listing instructions in current VM's program vector:");
          for instruction in self.vm.get_program() {
            println!("{:?}", instruction);
          }
          println!("End of program listing");
        }
        ".registers" => {
          println!("Listing contents of VM registers");
          println!("{:?}", self.vm.get_registers());
          println!("End of registers listing");
        }
        ".dump" => {
          println!("---- Printing VM dump ----");
          print!("{:?}", self.vm);
          println!("---- End printing VM dump");
        }
        ".symbols" => {
          println!("---- Printing symbols table ----");
          println!("{:?}", self.asm.symbols);
          println!("---- End printing symbols table ----");
        }
        ".clear" => {
          self.vm.clear();
          println!("Program vector cleared");
        }
        ".load_file" => {
          print!("Enter file path: ");
          io::stdout()
            .flush()
            .expect("Could not flush standard output");
          let mut tmp = String::new();
          stdin
            .read_line(&mut tmp)
            .expect("Unable to read line from user");
          let tmp = tmp.trim();
          let filename = Path::new(tmp);
          let mut f = File::open(Path::new(&filename)).expect("File not found");
          let mut contents = String::new();
          f.read_to_string(&mut contents).expect("Error reading file");
          let program = match program(CompleteStr(&contents)) {
            Ok((_, program)) => program,
            Err(e) => {
              println!("Unable to parse input: {:?}", e);
              continue;
            }
          };
          self.vm.program = program.to_bytes(&self.asm.symbols);
          self.vm.run();
        }
        _ => {
          let parsed_program = program(CompleteStr(&input));
          if !parsed_program.is_ok() {
            let results = self.parse_hex(&input);
            match results {
              Ok(bytes) => {
                for byte in bytes {
                  self.vm.add_byte(byte);
                }
              }
              Err(_e) => {
                println!("Unable to parse input");
              }
            }
          } else {
            let (_, result) = parsed_program.unwrap();
            let bytecode = result.to_bytes(&self.asm.symbols);
            for b in bytecode {
              self.vm.add_byte(b);
            }
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
        }
        Err(e) => {
          return Err(e);
        }
      }
    }

    Ok(results)
  }
}
