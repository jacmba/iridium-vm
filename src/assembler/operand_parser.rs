use super::label_parser::label_usage;
use super::register_parser::register;
use nom::digit;
use nom::types::CompleteStr;

use super::Token;

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
    register |
    label_usage |
    irstring
  )
);

named!(pub irstring <CompleteStr, Token>,
  do_parse!(
    tag!("'") >>
    content: take_until!("'") >>
    tag!("'") >>
    (
      Token::IrString{name: content.to_string()}
    )
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

  #[test]
  fn test_parse_string_operand() {
    let result = irstring(CompleteStr("'This is a test'"));
    assert!(result.is_ok());
    let (_, res_string) = result.unwrap();
    assert_eq!(
      res_string,
      Token::IrString {
        name: "This is a test".to_string()
      }
    );
  }
}
