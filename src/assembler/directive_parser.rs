use super::instruction_parser::AsmInstruction;
use super::label_parser::label_declaration;
use super::operand_parser::operand;
use super::Token;
use nom::alpha1;
use nom::types::CompleteStr;

named!(directive_declaration<CompleteStr, Token>,
  do_parse!(
    tag!(".") >>
    name: alpha1 >>
    (
      Token::Directive{name: name.to_string()}
    )
  )
);

named!(directive_combined<CompleteStr, AsmInstruction>,
  ws!(
    do_parse!(
      l: opt!(label_declaration) >>
      name: directive_declaration >>
      o1: opt!(operand) >>
      o2: opt!(operand) >>
      o3: opt!(operand) >>
      (
        AsmInstruction::new(Some(name), l, None, o1, o2, o3)
      )
    )
  )
);

named!(pub directive<CompleteStr, AsmInstruction>,
  do_parse!(
    ins: alt!(
      directive_combined
    ) >>
    (
      ins
    )
  )
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_directive() {
    let result = directive_combined(CompleteStr("test: .asciiz 'hello'"));
    assert!(result.is_ok());
    let (_, directive) = result.unwrap();

    let correct_instruction = AsmInstruction::new(
      Some(Token::Directive {
        name: "asciiz".to_string(),
      }),
      Some(Token::LabelDeclaration {
        name: "test".to_string(),
      }),
      None,
      Some(Token::IrString {
        name: "hello".to_string(),
      }),
      None,
      None,
    );

    assert_eq!(directive, correct_instruction);
  }
}
