#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
  HLT,
  LOAD,
  ADD,
  SUB,
  MUL,
  DIV,
  JMP,
  JMPF,
  JMPB,
  EQ,
  NEQ,
  GT,
  LT,
  GTE,
  LTE,
  JEQ,
  IGL,
}

pub struct Instruction {
  opcode: Opcode,
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction { opcode: opcode }
  }
}

impl From<u8> for Opcode {
  fn from(v: u8) -> Self {
    match v {
      0 => Opcode::HLT,
      1 => Opcode::LOAD,
      2 => Opcode::ADD,
      3 => Opcode::SUB,
      4 => Opcode::MUL,
      5 => Opcode::DIV,
      6 => Opcode::JMP,
      7 => Opcode::JMPF,
      8 => Opcode::JMPB,
      9 => Opcode::EQ,
      10 => Opcode::NEQ,
      11 => Opcode::GT,
      12 => Opcode::LT,
      13 => Opcode::GTE,
      14 => Opcode::LTE,
      15 => Opcode::JEQ,
      _ => Opcode::IGL,
    }
  }
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
    let opcode = Opcode::HLT;
    assert_eq!(opcode, Opcode::HLT);
  }

  #[test]
  fn test_create_instruction() {
    let instruction = Instruction::new(Opcode::HLT);
    assert_eq!(instruction.opcode, Opcode::HLT);
  }
}
