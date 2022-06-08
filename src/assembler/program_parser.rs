use super::instruction_parser::*;
use super::SymbolTable;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct Program {
  pub instructions: Vec<AsmInstruction>,
}

named!(pub program<CompleteStr, Program>,
  do_parse!(
    instructions: many1!(instruction) >>
    (
      Program {
        instructions: instructions
      }
    )
  )
);

impl Program {
  pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
    let mut program: Vec<u8> = vec![];

    for inst in &self.instructions {
      program.append(&mut inst.to_bytes(symbols));
    }

    program
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_program() {
    let result = program(CompleteStr("ld $0 #100\nld $1 #55"));
    assert!(result.is_ok());
    let (leftover, p) = result.unwrap();
    assert_eq!(leftover, CompleteStr(""));
    assert_eq!(p.instructions.len(), 2);
  }

  #[test]
  fn test_program_to_bytes() {
    let result = program(CompleteStr("ld $1 #100"));
    assert!(result.is_ok());
    let (_, prg) = result.unwrap();
    let symbols = SymbolTable::new();
    let bytes = prg.to_bytes(&symbols);
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes, vec![1, 1, 0, 100]);
  }

  #[test]
  fn test_program_with_logical_op_to_bytes() {
    let result = program(CompleteStr("eq $0 $1"));
    assert!(result.is_ok());
    let (_, prg) = result.unwrap();
    let symbols = SymbolTable::new();
    let bytes = prg.to_bytes(&symbols);
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes, vec![9, 0, 1, 0]);
  }
}
