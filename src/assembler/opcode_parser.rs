use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;

named!(pub opcode_halt<CompleteStr, Token>,
  do_parse!(
    tag!("hlt") >> (Token::Op{code: Opcode::HLT})
  )
);

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
    tag!("ld") >> (Token::Op{code: Opcode::LOAD})
  )
);

//----------------------------------------------------------------------------------------------

mod tests {
  use super::*;

  #[test]
  fn test_opcode_parse_load() {
    let result = opcode_load(CompleteStr("ld"));
    assert_eq!(result.is_ok(), true);
    let (rest, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: Opcode::LOAD });
    assert_eq!(rest, CompleteStr(""));
  }

  #[test]
  fn test_opcode_parse_halt() {
    let result = opcode_halt(CompleteStr("hlt"));
    assert!(result.is_ok());
    let (rest, token) = result.unwrap();
    assert_eq!(token, Token::Op { code: Opcode::HLT });
    assert_eq!(rest, CompleteStr(""));
  }

  #[test]
  fn test_invalid_opcode() {
    let result = opcode_load(CompleteStr("invalid_thing"));
    assert_eq!(result.is_ok(), false);
  }
}
