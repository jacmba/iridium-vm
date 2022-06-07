use nom::types::CompleteStr;
use nom::{alphanumeric, multispace};

use super::Token;

named!(pub label_declaration<CompleteStr, Token>,
  ws!(
    do_parse!(
      name: alphanumeric >>
      tag!(":") >>
      opt!(multispace) >>
      (
        Token::LabelDeclaration{name: name.to_string()}
      )
    )
  )
);

named!(pub label_usage<CompleteStr, Token>,
  ws!(
    do_parse!(
      tag!("@") >>
      name: alphanumeric >>
      opt!(multispace) >>
      (
        Token::LabelUsage{name: name.to_string()}
      )
    )
  )
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_label_declaration() {
    let result = label_declaration(CompleteStr("test:"));
    assert!(result.is_ok());
    let (_, label) = result.unwrap();
    assert_eq!(
      label,
      Token::LabelDeclaration {
        name: "test".to_string()
      }
    );

    let result = label_declaration(CompleteStr("invalid_label"));
    assert!(!result.is_ok());
  }

  #[test]
  fn test_label_usage() {
    let result = label_usage(CompleteStr("@test"));
    assert!(result.is_ok());
    let (_, label) = result.unwrap();
    assert_eq!(
      label,
      Token::LabelUsage {
        name: "test".to_string()
      }
    );

    let result = label_usage(CompleteStr("invalid_label_usage:"));
    assert!(!result.is_ok());
  }
}
