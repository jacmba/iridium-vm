use super::directive_parser::directive;
use super::label_parser::*;
use super::opcode_parser::*;
use super::operand_parser::*;
use super::register_parser::register;
use super::Token;
use nom::multispace;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AsmInstruction {
  opcode: Option<Token>,
  operand1: Option<Token>,
  operand2: Option<Token>,
  operand3: Option<Token>,
  label: Option<Token>,
  directive: Option<Token>,
}

named!(pub instruction<CompleteStr, AsmInstruction>,
  do_parse!(
    ins: alt!(instruction_combined | directive) >>
    (
      ins
    )
  )
);

named!(pub instruction_combined<CompleteStr, AsmInstruction>,
  do_parse!(
    l: opt!(label_declaration) >>
    o: opcode >>
    o1: opt!(operand) >>
    o2: opt!(operand) >>
    o3: opt!(operand) >>
    (
      AsmInstruction{
        opcode: Some(o),
        label: l,
        directive: None,
        operand1: o1,
        operand2: o2,
        operand3: o3,
      }
    )
  )
);

impl AsmInstruction {
  pub fn new(
    directive: Option<Token>,
    label: Option<Token>,
    opcode: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
  ) -> AsmInstruction {
    AsmInstruction {
      directive: directive,
      label: label,
      opcode: opcode,
      operand1: operand1,
      operand2: operand2,
      operand3: operand3,
    }
  }

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
      Some(t) => match t {
        Token::Op { code } => {
          results.push(*code as u8);
        }
        _ => {
          println!("Non-opcode found in opcode field");
          std::process::exit(1);
        }
      },
      None => (),
    };

    for operand in &[&self.operand1, &self.operand2, &self.operand3] {
      if let Some(t) = operand {
        AsmInstruction::extract_operand(&t, &mut results)
      }
    }

    while results.len() < 4 {
      results.push(0);
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
    let result = instruction(CompleteStr("ld $0 #100"));
    assert_eq!(
      result,
      Ok((
        CompleteStr(""),
        AsmInstruction {
          opcode: Some(Token::Op { code: Opcode::LOAD }),
          operand1: Some(Token::Register { reg_num: 0 }),
          operand2: Some(Token::IntegerOperand { value: 100 }),
          operand3: None,
          label: None,
          directive: None
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
      opcode: Some(Token::Op { code: Opcode::HLT }),
      operand1: None,
      operand2: None,
      operand3: None,
      label: None,
      directive: None,
    };

    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res[0], 0);
  }

  #[test]
  fn test_load_with_operands() {
    let inst = AsmInstruction {
      opcode: Some(Token::Op { code: Opcode::LOAD }),
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: Some(Token::IntegerOperand { value: 50 }),
      operand3: None,
      label: None,
      directive: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res, vec![1, 0, 0, 50]);
  }

  #[test]
  fn test_add_with_all_registers() {
    let inst = AsmInstruction {
      opcode: Some(Token::Op { code: Opcode::ADD }),
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: Some(Token::Register { reg_num: 1 }),
      operand3: Some(Token::Register { reg_num: 2 }),
      label: None,
      directive: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res, vec![2, 0, 1, 2]);
  }

  #[test]
  fn test_logical_operation_with_two_registers() {
    let inst = AsmInstruction {
      opcode: Some(Token::Op { code: Opcode::EQ }),
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: Some(Token::Register { reg_num: 1 }),
      operand3: None,
      label: None,
      directive: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res, vec![9, 0, 1, 0]);
  }

  #[test]
  fn test_mem_operation_with_one_register() {
    let inst = AsmInstruction {
      opcode: Some(Token::Op { code: Opcode::ALOC }),
      operand1: Some(Token::Register { reg_num: 0 }),
      operand2: None,
      operand3: None,
      label: None,
      directive: None,
    };
    let res = inst.to_bytes();
    assert_eq!(res.len(), 4);
    assert_eq!(res, vec![16, 0, 0, 0]);
  }
}
