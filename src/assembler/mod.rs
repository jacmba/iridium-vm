use super::instruction::Opcode;

pub mod directive_parser;
pub mod instruction_parser;
pub mod label_parser;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
  Op { code: Opcode },
  Register { reg_num: u8 },
  IntegerOperand { value: i32 },
  LabelDeclaration { name: String },
  LabelUsage { name: String },
  Directive { name: String },
}
