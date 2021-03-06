use nom::types::CompleteStr;

named!(opcode_load<CompleteStr, Token>,
  do_parse!(
    tag!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);

//----------------------------------------------------------------------------------------------

mod tests {
  use super::*;

  #[test]
  fn test_opcode_parse_load() {
      let result = opcode_load(CompleteStr("load"));
      assert_eq!(result.is_ok(), true);
      let (rest, token) = result.unwrap();
      assert_eq!(token, Token::Op{code: Opcode::LOAD});
      assert_eq!(rest, CompleteStr(""));
  }
}
