use crate::instruction::Opcode;

pub struct VM {
  registers: [i32; 32],
  pc: usize,
  program: Vec<u8>,
  reminder: u32
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      program: vec![],
      pc: 0,
      reminder: 0
    }
  }

  pub fn run(&mut self) {
    loop {
      if self.pc >= self.program.len() {
        break;
      }

      match self.decode_opcode() {
          Opcode::HLT => {
            println!("HLT encountered");
            return;
          },
          Opcode::LOAD => {
            let register = self.next_8_bits() as usize;
            let number = self.next_16_bits() as u16;
            self.registers[register] = number as i32;
            continue;
          },
          Opcode::ADD => {
            let r1 = self.registers[self.next_8_bits() as usize];
            let r2 = self.registers[self.next_8_bits() as usize];
            let r3 = self.next_8_bits() as usize;
            self.registers[r3] = r1 + r2;
          },
          Opcode::SUB => {
            let r1 = self.registers[self.next_8_bits() as usize];
            let r2 = self.registers[self.next_8_bits() as usize];
            let r3 = self.next_8_bits() as usize;
            self.registers[r3] = r1 - r2;
          },
          Opcode::MUL => {
            let r1 = self.registers[self.next_8_bits() as usize];
            let r2 = self.registers[self.next_8_bits() as usize];
            let r3 = self.next_8_bits() as usize;
            self.registers[r3] = r1 * r2;
          },
          Opcode::DIV => {
            let r1 = self.registers[self.next_8_bits() as usize];
            let r2 = self.registers[self.next_8_bits() as usize];
            let r3 = self.next_8_bits() as usize;
            self.registers[r3] = r1 / r2;
            self.reminder = (r1 % r2) as u32;
          }
          _ => {
            println!("Unrecognized opcode found! Terminating...");
            return;
          },
      }
    }
  }

  fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    result
  }

  fn next_16_bits(&mut self) -> u16 {
    let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
    self.pc += 2;
    result
  }

  fn decode_opcode(&mut self) -> Opcode {
    let opcode = Opcode::from(self.program[self.pc]);
    self.pc += 1;
    opcode
  }
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_vm() {
    let test_vm = VM::new();
    assert_eq!(test_vm.registers[0], 0)
  }

  #[test]
  fn test_opcode_hlt() {
      let mut test_vm = VM::new();
      let test_bytes = vec![0, 0, 0, 0];
      test_vm.program = test_bytes;
      test_vm.run();
      assert_eq!(test_vm.pc, 1);
  }
  
  #[test]
  fn test_opcode_igl() {
      let mut test_vm = VM::new();
      let test_bytes = vec![200, 0, 0, 0];
      test_vm.program = test_bytes;
      test_vm.run();
      assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_load() {
      let mut test_vm = VM::new();
      test_vm.program = vec![1, 0, 2, 246];
      test_vm.run();
      assert_eq!(test_vm.registers[0], 758);
  }

  #[test]
  fn test_opcode_add() {
      let mut test_vm = VM::new();
      test_vm.program = vec![2, 0, 1, 2];
      test_vm.registers[0] = 2;
      test_vm.registers[1] = 3;
      test_vm.run();
      assert_eq!(test_vm.registers[2], 5);
  }

  #[test]
  fn test_opcode_sub() {
      let mut test_vm = VM::new();
      test_vm.program = vec![3, 0, 1, 2];
      test_vm.registers[0] = 8;
      test_vm.registers[1] = 3;
      test_vm.run();
      assert_eq!(test_vm.registers[2], 5);
  }

  #[test]
  fn test_opcode_mul() {
      let mut test_vm = VM::new();
      test_vm.program = vec![4, 0, 1, 2];
      test_vm.registers[0] = 8;
      test_vm.registers[1] = 3;
      test_vm.run();
      assert_eq!(test_vm.registers[2], 24);
  }

  #[test]
  fn test_opcode_div() {
      let mut test_vm = VM::new();
      test_vm.program = vec![5, 0, 1, 2];
      test_vm.registers[0] = 11;
      test_vm.registers[1] = 3;
      test_vm.run();
      assert_eq!(test_vm.registers[2], 3);
      assert_eq!(test_vm.reminder, 2);
  }
}
