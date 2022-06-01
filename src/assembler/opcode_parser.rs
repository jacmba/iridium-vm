use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;

named!(opcode_load<CompleteStr, Token>,
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
  fn test_invalid_opcode() {
    let result = opcode_load(CompleteStr("invalid_thing"));
    assert_eq!(result.is_ok(), false);
  }
}
