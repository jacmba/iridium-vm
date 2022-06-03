use crate::assembler::opcode_parser::*;
use crate::assembler::operand_parser::integer_operand;
use crate::assembler::register_parser::register;
use crate::assembler::Token;
use nom::multispace;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AsmInstruction {
  opcode: Token,
  operand1: Option<Token>,
  operand2: Option<Token>,
  operand3: Option<Token>,
}

named!(instruction_one<CompleteStr, AsmInstruction>,
  do_parse!(
    o: opcode >>
    r: register >>
    i: integer_operand >>
    (
      AsmInstruction {
        opcode: o,
        operand1: Some(r),
        operand2: Some(i),
        operand3: None
      }
    )
  )
);

named!(instruction_two<CompleteStr, AsmInstruction>,
  do_parse!(
    o: opcode >>
    r1: register >>
    r2: register >>
    r3: register >>
    (
      AsmInstruction {
        opcode: o,
        operand1: Some(r1),
        operand2: Some(r2),
        operand3: Some(r3)
      }
    )
  )
);

named!(instruction_three<CompleteStr, AsmInstruction>,
  do_parse!(
    o: opcode >>
    r1: register >>
    r2: register >>
    (
      AsmInstruction {
        opcode: o,
        operand1: Some(r1),
        operand2: Some(r2),
        operand3: None
      }
    )
  )
);

named!(instruction_four<CompleteStr, AsmInstruction>,
  do_parse!(
    o: opcode >>
    r: register >>
    (
      AsmInstruction {
        opcode: o,
        operand1: Some(r),
        operand2: None,
        operand3: None
      }
    )
  )
);

named!(instruction_five<CompleteStr, AsmInstruction>,
  do_parse!(
    o: opcode >>
    opt!(multispace) >>
    (
      AsmInstruction {
        opcode: o,
        operand1: None,
        operand2: None,
        operand3: None
      }
    )
  )
);

named!(pub instruction<CompleteStr, AsmInstruction>,
  do_parse!(
    ins: alt!(instruction_one | instruction_two |
      instruction_three | instruction_four | instruction_five) >>
    (
      ins
    )
  )
);

impl AsmInstruction {
  pub fn extract_operand(t: &Token, results: &mut Vec<u8>) {
    match t {
      Token::Register { reg_num } => {
        results.push(*reg_num);
      }
      Token::IntegerOperand { value } => {
        let converted = *value as u16;
        let byte1 = converted;
        let byte2 = converted >> 8;
        results.push(byte2 as u8);
        results.push(byte1 as u8);
      }
      _ => {
        println!("Opcode found in operand field");
        std::process::exit(1);
      }
    };
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut results: Vec<u8> = vec![];
    match &self.opcode {
      Token::Op { code } => {
        results.push(*code as u8);
      }
      _ => {
        println!("Non-opcode found in opcode field");
        std::process::exit(1);
      }
    };

    for operand in &[&self.operand1, &self.operand2, &self.operand3] {
      if let Some(t) = operand {
        AsmInstruction::extract_operand(&t, &mut results)
      }
    }

    results
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::instruction::Opcode;

  #[test]
  fn test_parse_instruction_form_one() {
    let result = instruction_one(CompleteStr("ld $0 #100"));
    assert_eq!(
      result,
      Ok((
        CompleteStr(""),
        AsmInstruction {
          opcode: Token::Op { code: Opcode::LOAD },
          operand1: Some(Token::Register { reg_num: 0 }),
          operand2: Some(Token::IntegerOperand { value: 100 }),
          operand3: None
        }
      ))
    );
  }

  #[test]
  fn test_extract_register_operand() {
    let tok = Token::Register { reg_num: 5 };
    let mut v: Vec<u8> = vec![];
    AsmInstruction::extract_operand(&tok, &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 5);
  }

  #[test]
  fn test_extract_integer_operand() {
    let tok = Token::IntegerOperand { value: 255 };
    let mut v: Vec<u8> = vec![];
    AsmInstruction::extract_operand(&tok, &mut v);
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 0);
    assert_eq!(v[1], 255);
  }

  #[test]
  fn test_halt_function_single_opcode_byte() {
    let inst = AsmInstruction {
      opcode: Token::Op { code: Opcode::HLT },
      operand1: None,
      operand2: None,
      operand3: None,
    };

    let res = inst.to_bytes();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], 0);
  }

  #[test]
  fn test_load_with_operands() {
    let inst = AsmInstruction {
      opcode: Token::Op { code: Opcode::LOAD },
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: Some(Token::IntegerOperand { value: 50 }),
      operand3: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res, vec![1, 0, 0, 50]);
  }

  #[test]
  fn test_add_with_all_registers() {
    let inst = AsmInstruction {
      opcode: Token::Op { code: Opcode::ADD },
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: Some(Token::Register { reg_num: 1 }),
      operand3: Some(Token::Register { reg_num: 2 }),
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res, vec![2, 0, 1, 2]);
  }

  #[test]
  fn test_logical_operation_with_two_registers() {
    let inst = AsmInstruction {
      opcode: Token::Op { code: Opcode::EQ },
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: Some(Token::Register { reg_num: 1 }),
      operand3: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 3);
    assert_eq!(res, vec![9, 0, 1]);
  }

  #[test]
  fn test_mem_operation_with_one_register() {
    let inst = AsmInstruction {
      opcode: Token::Op { code: Opcode::ALOC },
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: None,
      operand3: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 2);
    assert_eq!(res, vec![16, 0]);
  }
}
