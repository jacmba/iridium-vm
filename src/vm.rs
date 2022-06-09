use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
  pub registers: [i32; 32],
  pc: usize,
  pub program: Vec<u8>,
  heap: Vec<u8>,
  reminder: u32,
  equal_flag: bool,
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      program: vec![],
      pc: 0,
      heap: vec![],
      reminder: 0,
      equal_flag: false,
    }
  }

  pub fn get_program(&self) -> &Vec<u8> {
    &self.program
  }

  pub fn get_registers(&self) -> &[i32] {
    &self.registers
  }

  pub fn get_pc(&self) -> usize {
    self.pc
  }

  pub fn add_byte(&mut self, byte: u8) {
    self.program.push(byte);
  }

  pub fn run(&mut self) {
    let mut running = true;
    while running {
      running = self.execute_instruction();
    }
  }

  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  pub fn clear(&mut self) {
    self.program = vec![];
    self.pc = 0;
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

  fn execute_instruction(&mut self) -> bool {
    if self.pc >= self.program.len() {
      return false;
    }

    let opcode = self.decode_opcode();
    match opcode {
      // Machine halting
      Opcode::HLT => {
        println!("HLT encountered");
        return false;
      }

      // Register load
      Opcode::LOAD => {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u16;
        self.registers[register] = number as i32;
      }

      // Arithmetic ops
      Opcode::ADD => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        let r3 = self.next_8_bits() as usize;
        self.registers[r3] = r1 + r2;
      }
      Opcode::SUB => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        let r3 = self.next_8_bits() as usize;
        self.registers[r3] = r1 - r2;
      }
      Opcode::MUL => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        let r3 = self.next_8_bits() as usize;
        self.registers[r3] = r1 * r2;
      }
      Opcode::DIV => {
        let r1 = self.registers[self.next_8_bits() as usize];
        let r2 = self.registers[self.next_8_bits() as usize];
        let r3 = self.next_8_bits() as usize;
        self.registers[r3] = r1 / r2;
        self.reminder = (r1 % r2) as u32;
      }
      Opcode::INC => {
        let r = self.next_8_bits() as usize;
        self.registers[r] += 1;
        self.pc += 2;
      }
      Opcode::DEC => {
        let r = self.next_8_bits() as usize;
        self.registers[r] -= 1;
        self.pc += 2;
      }

      // Jumps
      Opcode::JMP => {
        self.pc = self.registers[self.next_8_bits() as usize] as usize;
      }
      Opcode::JMPF => {
        self.pc += self.registers[self.next_8_bits() as usize] as usize;
      }
      Opcode::JMPB => {
        self.pc -= self.registers[self.next_8_bits() as usize] as usize;
      }

      // Logic comparisons
      Opcode::EQ => {
        let l = self.registers[self.next_8_bits() as usize];
        let r = self.registers[self.next_8_bits() as usize];
        self.equal_flag = l == r;
        self.pc += 1;
      }
      Opcode::NEQ => {
        let l = self.registers[self.next_8_bits() as usize];
        let r = self.registers[self.next_8_bits() as usize];
        self.equal_flag = l != r;
        self.pc += 1;
      }
      Opcode::GT => {
        let l = self.registers[self.next_8_bits() as usize];
        let r = self.registers[self.next_8_bits() as usize];
        self.equal_flag = l > r;
        self.pc += 1;
      }
      Opcode::LT => {
        let l = self.registers[self.next_8_bits() as usize];
        let r = self.registers[self.next_8_bits() as usize];
        self.equal_flag = l < r;
        self.pc += 1;
      }
      Opcode::GTE => {
        let l = self.registers[self.next_8_bits() as usize];
        let r = self.registers[self.next_8_bits() as usize];
        self.equal_flag = l >= r;
        self.pc += 1;
      }
      Opcode::LTE => {
        let l = self.registers[self.next_8_bits() as usize];
        let r = self.registers[self.next_8_bits() as usize];
        self.equal_flag = l <= r;
        self.pc += 1;
      }
      Opcode::JEQ => {
        if self.equal_flag {
          self.pc = self.registers[self.next_8_bits() as usize] as usize;
        } else {
          self.pc += 3;
        }
      }

      // Memory
      Opcode::ALOC => {
        let reg = self.next_8_bits() as usize;
        let bytes = self.registers[reg];
        let new_end = self.heap.len() as i32 + bytes;
        self.heap.resize(new_end as usize, 0);
        self.pc += 2;
      }

      // Invalid code
      _ => {
        println!("Unrecognized opcode [{:?}] found! Terminating...", opcode);
        return false;
      }
    }
    true
  }
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  fn get_test_vm() -> VM {
    VM::new()
  }

  #[test]
  fn test_opcode_hlt() {
    let mut test_vm = get_test_vm();
    let test_bytes = vec![0, 0, 0, 0];
    test_vm.program = test_bytes;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 1);
  }
  #[test]
  fn test_opcode_igl() {
    let mut test_vm = get_test_vm();
    let test_bytes = vec![200, 0, 0, 0];
    test_vm.program = test_bytes;
    test_vm.run_once();
    assert_eq!(test_vm.pc, 1);
  }

  #[test]
  fn test_opcode_load() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![1, 0, 2, 246];
    test_vm.run_once();
    assert_eq!(test_vm.registers[0], 758);
  }

  #[test]
  fn test_opcode_add() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![2, 0, 1, 2];
    test_vm.registers[0] = 2;
    test_vm.registers[1] = 3;
    test_vm.run_once();
    assert_eq!(test_vm.registers[2], 5);
  }

  #[test]
  fn test_opcode_sub() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![3, 0, 1, 2];
    test_vm.registers[0] = 8;
    test_vm.registers[1] = 3;
    test_vm.run_once();
    assert_eq!(test_vm.registers[2], 5);
  }

  #[test]
  fn test_opcode_mul() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![4, 0, 1, 2];
    test_vm.registers[0] = 8;
    test_vm.registers[1] = 3;
    test_vm.run_once();
    assert_eq!(test_vm.registers[2], 24);
  }

  #[test]
  fn test_opcode_div() {
    let mut test_vm = get_test_vm();
    test_vm.program = vec![5, 0, 1, 2];
    test_vm.registers[0] = 11;
    test_vm.registers[1] = 3;
    test_vm.run_once();
    assert_eq!(test_vm.registers[2], 3);
    assert_eq!(test_vm.reminder, 2);
  }

  #[test]
  fn test_opcode_jmp() {
    let mut vm = get_test_vm();
    vm.program = vec![6, 8, 0, 0];
    vm.registers[8] = 2;
    vm.run_once();
    assert_eq!(vm.pc, 2);
  }

  #[test]
  fn test_opcode_jmp_f_b() {
    let mut vm = get_test_vm();
    vm.program = vec![7, 0, 0, 0, 8, 1];
    vm.registers[0] = 2;
    vm.registers[1] = 3;
    vm.run_once();
    assert_eq!(vm.pc, 4);
    vm.run_once();
    assert_eq!(vm.pc, 3);
  }

  #[test]
  fn test_opcode_eq() {
    let mut vm = get_test_vm();
    vm.program = vec![9, 0, 1, 0, 9, 0, 2];
    vm.registers[0] = 5;
    vm.registers[1] = 5;
    vm.registers[2] = 7;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_opcode_neq() {
    let mut vm = get_test_vm();
    vm.program = vec![10, 0, 1, 0, 10, 0, 2];
    vm.registers[0] = 3;
    vm.registers[1] = 4;
    vm.registers[2] = 3;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_opcode_gt() {
    let mut vm = get_test_vm();
    vm.program = vec![11, 0, 1, 0, 11, 0, 2];
    vm.registers[0] = 2;
    vm.registers[1] = 1;
    vm.registers[2] = 2;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_opcode_lt() {
    let mut vm = get_test_vm();
    vm.program = vec![12, 0, 1, 0, 12, 0, 2];
    vm.registers[0] = 5;
    vm.registers[1] = 10;
    vm.registers[2] = 5;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_opcode_gte() {
    let mut vm = get_test_vm();
    vm.program = vec![13, 0, 1, 0, 13, 0, 2, 0, 13, 0, 3, 0];
    vm.registers[0] = 4;
    vm.registers[1] = 4;
    vm.registers[2] = 2;
    vm.registers[3] = 7;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_opcode_lte() {
    let mut vm = get_test_vm();
    vm.program = vec![14, 0, 1, 0, 14, 0, 2, 0, 14, 0, 3];
    vm.registers[0] = 5;
    vm.registers[1] = 5;
    vm.registers[2] = 10;
    vm.registers[3] = 2;
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, true);
    vm.run_once();
    assert_eq!(vm.equal_flag, false);
  }

  #[test]
  fn test_opcode_jeq() {
    let mut vm = get_test_vm();
    vm.program = vec![15, 0, 0, 0];
    vm.equal_flag = true;
    vm.registers[0] = 3;
    vm.run_once();
    assert_eq!(vm.pc, 3);
  }

  #[test]
  fn test_opcode_aloc() {
    let mut vm = get_test_vm();
    vm.registers[0] = 1024;
    vm.program = vec![16, 0, 0, 0];
    vm.run_once();
    assert_eq!(vm.heap.len(), 1024);
  }

  #[test]
  fn test_opcode_inc() {
    let mut vm = get_test_vm();
    vm.registers[0] = 1;
    vm.program = vec![17, 0];
    vm.run_once();
    assert_eq!(vm.registers[0], 2);
  }

  #[test]
  fn test_opcode_dec() {
    let mut vm = get_test_vm();
    vm.registers[0] = 2;
    vm.program = vec![18, 0];
    vm.run_once();
    assert_eq!(vm.registers[0], 1);
  }
}
