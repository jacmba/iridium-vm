use super::register_parser::register;
use nom::digit;
use nom::types::CompleteStr;

use crate::assembler::Token;

named!(pub integer_operand<CompleteStr, Token>,
  ws!(
    do_parse!(
      tag!("#") >>
      reg_num: digit >>
      (
        Token::IntegerOperand {value: reg_num.parse::<i32>().unwrap()}
      )
    )
  )
);

named!(pub operand<CompleteStr, Token>,
  alt!(
    integer_operand |
    register
  )
);

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_parse_integer_operand() {
    let result = integer_operand(CompleteStr("#10"));
    assert!(result.is_ok());
    let (rest, value) = result.unwrap();
    assert_eq!(rest, CompleteStr(""));
    assert_eq!(value, Token::IntegerOperand { value: 10 });
  }

  #[test]
  fn test_parse_invalid_integer_operands() {
    let result = integer_operand(CompleteStr("10"));
    assert!(!result.is_ok());

    let result = integer_operand(CompleteStr("#a"));
    assert!(!result.is_ok());
  }
}
