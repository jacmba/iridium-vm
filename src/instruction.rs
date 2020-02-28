#[derive(Debug, PartialEq)]
pub enum Opcode {
  HLT,
  IGL
}

pub struct Instruction {
  opcode: Opcode
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction {
      opcode: opcode
    }
  }
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
      let opCode = Opcode::HLT;
  }
}
