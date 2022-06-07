use super::instruction_parser::AsmInstruction;
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
      tag!(".") >>
      name: directive_declaration >>
      o1: opt!(operand) >>
      o2: opt!(operand) >>
      o3: opt!(operand) >>
      (
        AsmInstruction::new(Some(name), None, None, o1, o2, o3)
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
