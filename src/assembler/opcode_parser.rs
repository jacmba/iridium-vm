use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::alpha1;
use nom::types::CompleteStr;

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
    opcode: alpha1 >>
    (
      Token::Op{code: Opcode::from(opcode)}
    )
  )
);

//----------------------------------------------------------------------------------------------

mod tests {
  use super::*;

  #[test]
  fn test_opcode_parse_load() {
    let result = opcode(CompleteStr("ld"));
    assert_eq!(result.is_ok(), true);
    let (rest, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: Opcode::LOAD });
    assert_eq!(rest, CompleteStr(""));
  }

  #[test]
  fn test_opcode_parse_halt() {
    let result = opcode(CompleteStr("hlt"));
    assert!(result.is_ok());
    let (rest, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: Opcode::HLT });
    assert_eq!(rest, CompleteStr(""));
  }

  #[test]
  fn test_invalid_opcode() {
    let result = opcode(CompleteStr("invalid_thing"));
    let (_, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: Opcode::IGL });
  }
}
