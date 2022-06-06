use nom::types::CompleteStr;

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
  ALOC,
  INC,
  DEC,
  IGL,
}

pub struct Instruction {
  opcode: Opcode,
}

impl<'a> From<CompleteStr<'a>> for Opcode {
  fn from(v: CompleteStr<'a>) -> Self {
    match v {
      CompleteStr("hlt") => Opcode::HLT,
      CompleteStr("ld") => Opcode::LOAD,
      CompleteStr("add") => Opcode::ADD,
      CompleteStr("sub") => Opcode::SUB,
      CompleteStr("mul") => Opcode::MUL,
      CompleteStr("div") => Opcode::DIV,
      CompleteStr("jmp") => Opcode::JMP,
      CompleteStr("jmpf") => Opcode::JMPF,
      CompleteStr("jmpb") => Opcode::JMPB,
      CompleteStr("eq") => Opcode::EQ,
      CompleteStr("neq") => Opcode::NEQ,
      CompleteStr("gt") => Opcode::GT,
      CompleteStr("lt") => Opcode::LT,
      CompleteStr("gte") => Opcode::GTE,
      CompleteStr("lte") => Opcode::LTE,
      CompleteStr("jeq") => Opcode::JEQ,
      CompleteStr("aloc") => Opcode::ALOC,
      CompleteStr("inc") => Opcode::INC,
      CompleteStr("dec") => Opcode::DEC,
      _ => Opcode::IGL,
    }
  }
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
      16 => Opcode::ALOC,
      17 => Opcode::INC,
      18 => Opcode::DEC,
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

  #[test]
  fn test_create_hlt_opcode_from_string() {
    let op = Opcode::from(CompleteStr("hlt"));
    assert_eq!(op, Opcode::HLT);
  }

  #[test]
  fn test_logical_opcodes_from_string() {
    let op = Opcode::from(CompleteStr("jmp"));
    assert_eq!(op, Opcode::JMP);
  }

  #[test]
  fn test_parse_invalid_opcode_from_string() {
    let op = Opcode::from(CompleteStr("invalid one"));
    assert_eq!(op, Opcode::IGL);
  }
}
