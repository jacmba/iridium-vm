use super::instruction::Opcode;

pub mod opcode_parser;
pub mod register_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
  Op { code: Opcode },
  Register { reg_num: u8 },
}
