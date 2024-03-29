pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

use super::instruction::Opcode;
use nom::types::CompleteStr;

pub mod directive_parser;
pub mod instruction_parser;
pub mod label_parser;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

use instruction_parser::*;
use program_parser::*;

#[derive(Debug, PartialEq)]
pub enum Token {
  Op { code: Opcode },
  Register { reg_num: u8 },
  IntegerOperand { value: i32 },
  LabelDeclaration { name: String },
  LabelUsage { name: String },
  Directive { name: String },
  IrString { name: String },
}

#[derive(Debug, PartialEq)]
pub enum AssemblerPhase {
  First,
  Second,
}

impl Default for AssemblerPhase {
  fn default() -> Self {
    AssemblerPhase::First
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
  Data { starting_instruction: Option<u32> },
  Code { starting_instruction: Option<u32> },
  Unknown,
}

impl Default for AssemblerSection {
  fn default() -> Self {
    AssemblerSection::Unknown
  }
}

impl<'a> From<&'a str> for AssemblerSection {
  fn from(name: &str) -> AssemblerSection {
    match name {
      "data" => AssemblerSection::Data {
        starting_instruction: None,
      },
      "code" => AssemblerSection::Code {
        starting_instruction: None,
      },
      _ => AssemblerSection::Unknown,
    }
  }
}

#[derive(Debug)]
pub enum SymbolType {
  Label,
}

#[derive(Debug)]
pub struct Symbol {
  name: String,
  offset: u32,
  symbol_type: SymbolType,
}

impl Symbol {
  pub fn new(name: String, offset: u32, symbol_type: SymbolType) -> Symbol {
    Symbol {
      name: name,
      offset: offset,
      symbol_type: symbol_type,
    }
  }
}

#[derive(Debug)]
pub struct SymbolTable {
  symbols: Vec<Symbol>,
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    SymbolTable { symbols: vec![] }
  }

  pub fn add_symbol(&mut self, symbol: Symbol) {
    self.symbols.push(symbol);
  }

  pub fn symbol_value(&self, s: &str) -> Option<u32> {
    for symbol in &self.symbols {
      if symbol.name == s {
        return Some(symbol.offset);
      }
    }
    None
  }

  pub fn set_symbol_offset(&mut self, s: &str, offset: u32) -> bool {
    for symbol in &mut self.symbols {
      if symbol.name == s {
        symbol.offset = offset;
        return true;
      }
    }
    false
  }
}

#[derive(Debug)]
pub struct Assembler {
  pub phase: AssemblerPhase,
  pub symbols: SymbolTable,
  pub ro: Vec<u8>,
  pub bytecode: Vec<u8>,
  ro_offset: u32,
  sections: Vec<AssemblerSection>,
  current_section: Option<AssemblerSection>,
  current_instruction: u32,
}

impl Assembler {
  pub fn new() -> Assembler {
    Assembler {
      phase: AssemblerPhase::First,
      symbols: SymbolTable::new(),
      ro: vec![],
      bytecode: vec![],
      ro_offset: 0,
      sections: vec![],
      current_section: None,
      current_instruction: 0,
    }
  }

  fn extract_labels(&mut self, p: &Program) {
    let mut c = 0;
    for i in &p.instructions {
      if i.is_label() {
        match i.label_name() {
          Some(name) => {
            let symbol = Symbol::new(name, c, SymbolType::Label);
            self.symbols.add_symbol(symbol);
          }
          None => {}
        }
      }

      c += 4;
    }
  }

  fn process_first_phase(&mut self, p: &Program) {
    self.extract_labels(p);
    self.phase = AssemblerPhase::Second;
  }

  fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
    let mut program = vec![];
    for i in &p.instructions {
      if i.is_opcode() {
        let mut bytes = i.to_bytes(&self.symbols);
        program.append(&mut bytes);
      }
      if i.is_directive() {
        self.process_directive(i);
      }

      self.current_instruction += 1;
    }
    program
  }

  pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
    match program(CompleteStr(raw)) {
      Ok((_, program)) => {
        let mut assembled_program = self.write_pie_header();
        self.process_first_phase(&program);

        let mut body = self.process_second_phase(&program);

        if self.sections.len() < 2 {
          println!("Did not found at least 2 sections");
          // std::process::exit(1);
        }

        assembled_program.append(&mut body);
        Some(assembled_program)
      }
      Err(e) => {
        println!("Error assembling code: {:?}", e);
        None
      }
    }
  }

  fn write_pie_header(&self) -> Vec<u8> {
    let mut header = vec![];
    for byte in PIE_HEADER_PREFIX.iter() {
      header.push(byte.clone());
    }
    while header.len() < PIE_HEADER_LENGTH {
      header.push(0 as u8);
    }
    header
  }

  fn process_directive(&mut self, i: &AsmInstruction) {
    let directive_name = match i.directive_name() {
      Some(d) => d,
      None => {
        println!("Directive has invalid name: {:?}", i);
        return;
      }
    };

    if i.has_operands() {
      match directive_name.as_ref() {
        "asciiz" => {
          self.handle_asciiz(i);
        }
        _ => {
          println!("Unknown directive {}", directive_name);
        }
      }
    } else {
      self.process_section_header(&directive_name);
    }
  }

  fn process_section_header(&mut self, header_name: &str) {
    let new_section: AssemblerSection = header_name.into();
    if new_section == AssemblerSection::Unknown {
      println!("Found unkown section header: {:#?}", header_name);
      return;
    }

    self.sections.push(new_section.clone());
    self.current_section = Some(new_section);
  }

  fn handle_asciiz(&mut self, i: &AsmInstruction) {
    if self.phase != AssemblerPhase::First {
      return;
    }

    match i.get_string_constant() {
      Some(s) => {
        match i.label_name() {
          Some(name) => self.symbols.set_symbol_offset(&name, self.ro_offset),
          None => {
            println!("Found string with no associated label");
            return;
          }
        };
        for byte in s.as_bytes() {
          self.ro.push(*byte);
          self.ro_offset += 1;
        }
        self.ro.push(0);
        self.ro_offset += 1;
      }
      None => {
        println!("Empty string constant after .asciiz");
      }
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_symbol_table() {
    let mut sym = SymbolTable::new();
    let new_symbol = Symbol::new("test".to_string(), 32, SymbolType::Label);
    sym.add_symbol(new_symbol);
    let value = sym.symbol_value("test");
    assert!(value.is_some());
    let value = sym.symbol_value("wrong");
    assert!(value.is_none());
  }

  #[test]
  fn test_assemble_program() {
    let mut asm = Assembler::new();
    let test_string =
      "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
    let program = asm.assemble(test_string).unwrap();
    assert_eq!(program.len(), 92);
  }
}
