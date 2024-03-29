use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate clap;

use clap::App;

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let target_file = matches.value_of("INPUT_FILE");
  match target_file {
    Some(filename) => {
      let program = read_file(filename);
      let mut asm = assembler::Assembler::new();
      let mut vm = vm::VM::new();
      let program = asm.assemble(&program);
      match program {
        Some(p) => {
          vm.program = p;
          vm.run();
          print_stars();
          println!("Finished running {}. VM status: {:?}", filename, vm);
          println!("Symbols table: {:?}", asm.symbols);
          print_stars();
          std::process::exit(0);
        }
        None => {}
      };
    }
    None => {
      start_repl();
    }
  };
}

fn start_repl() {
  let mut repl = repl::REPL::new();
  repl.run();
}

fn read_file(tmp: &str) -> String {
  let filename = Path::new(tmp);
  match File::open(Path::new(&filename)) {
    Ok(mut fh) => {
      let mut contents = String::new();
      match fh.read_to_string(&mut contents) {
        Ok(_) => {
          return contents;
        }
        Err(e) => {
          println!("Error reading file: {:?}", e);
          std::process::exit(1);
        }
      }
    }
    Err(e) => {
      println!("File not found: {:?}", e);
      std::process::exit(1);
    }
  };
}

fn print_stars() {
  println!("**********************************************************************");
}
