use crate::instruction::Opcode;

/// Virtual machine state.
#[allow(dead_code)]
pub struct VM {
    /// VM registers.
    pub registers: [i32; 32],
    /// Program counter.
    pc: usize,
    /// Contains program bytecode.
    pub program: Vec<u8>,
    /// Memory heap.
    heap: Vec<u8>,
    /// Contains a remainder of module division operations.
    remainder: u32,
    /// Contains the result of the last comparison operation.
    equal_flag: bool,
}

impl Default for VM {
    fn default() -> Self {
        VM {
            registers: [0; 32],
            program: vec![],
            heap: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }
}

impl VM {
    /// Initializes a fresh VM state.
    pub fn new() -> VM {
        VM::default()
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn add_bytes(&mut self, mut bytes: Vec<u8>) {
        self.program.append(&mut bytes);
    }

    /// Runs the VM.
    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }

    /// Performs a single step of VM execution.
    pub fn step(&mut self) {
        self.execute_instruction();
    }

    /// Performs a number of VM execution steps.
    pub fn step_times(&mut self, n: usize) {
        for _ in 0..n {
            self.execute_instruction();
        }
    }

    /// Executes current VM instruction.
    ///
    /// Note that our virtual CPU always reads 16 or 32 bits of data at a time.
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::NOP => {}
            Opcode::LOAD => {
                let reg = self.next_8();
                let num = self.next_16();
                self.registers[reg] = num as i32;
            }
            Opcode::ALLOC => {
                let reg = self.next_8();
                let bytes = self.registers[reg];
                let size = self.heap.len() as i32 + bytes;
                self.heap.resize(size as usize, 0);
            }
            Opcode::ADD => {
                let reg1 = self.registers[self.next_8()];
                let reg2 = self.registers[self.next_8()];
                self.registers[self.next_8()] = reg1 + reg2;
            }
            Opcode::SUB => {
                let reg1 = self.registers[self.next_8()];
                let reg2 = self.registers[self.next_8()];
                self.registers[self.next_8()] = reg1 - reg2;
            }
            Opcode::MUL => {
                let reg1 = self.registers[self.next_8()];
                let reg2 = self.registers[self.next_8()];
                self.registers[self.next_8()] = reg1 * reg2;
            }
            Opcode::DIV => {
                let reg1 = self.registers[self.next_8()];
                let reg2 = self.registers[self.next_8()];
                self.registers[self.next_8()] = reg1 + reg2;
                self.remainder = (reg1 % reg2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8()];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let offset = self.registers[self.next_8()];
                self.pc += offset as usize;
            }
            Opcode::JMPB => {
                let offset = self.registers[self.next_8()];
                self.pc -= offset as usize;
            }
            Opcode::EQ => {
                let reg1 = self.registers[self.next_8()];
                let reg2 = self.registers[self.next_8()];
                self.equal_flag = reg1 == reg2;
                self.pc += 1;
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8()];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JNEQ => {
                let target = self.registers[self.next_8()];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::HLT => {
                println!("HLT encountered, stopping VM");
                return true;
            }
            Opcode::IGL => {
                println!("Unknown opcode, terminating VM");
                return true;
            }
        }
        false
    }

    /// Reads next byte as `usize`.
    fn next_8(&mut self) -> usize {
        let result = self.program[self.pc];
        self.pc += 1;
        result as usize
    }

    /// Reads next 2 bytes as `u16`.
    fn next_16(&mut self) -> u16 {
        let high = (self.program[self.pc] as u16) << 8;
        let low = self.program[self.pc + 1] as u16;
        self.pc += 2;
        high | low
    }

    /// Decodes and returns a current `Opcode` and
    /// increments a program counter.
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        // Once we have decoded the opcode, we want to move the
        // counter to the next byte
        self.pc += 1;
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_load() {
        let mut vm = VM::new();
        vm.program = vec![Opcode::LOAD.into(), 0, 1, 244];
        // >> (1 as u16) << 8
        // 256
        // 244 =  11110100
        // 256 = 100000000
        // >> 256 | 244
        // 500 = 111110100
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_alloc() {
        let mut vm = VM::new();
        vm.registers[0] = 1024;
        vm.program = vec![Opcode::ALLOC.into(), 0, 0, 0];
        vm.step();
        assert_eq!(vm.heap.len(), 1024);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.program = vec![0, 0, Opcode::JMP.into(), 0, 0];
        vm.step_times(3);
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        vm.program = vec![Opcode::JMPF.into(), 0, 0, 0, 6, 0, 0, 0];
        // pc  = 1 (in self.decode_opcode)
        // pc += 1 (in self.next_8_bits)
        vm.step();
        // pc += 2 (in JPMF opcode handler)
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_opcode_eq() {
        let mut vm = VM::new();
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.program = vec![Opcode::EQ.into(), 0, 1, 0, Opcode::EQ.into(), 0, 1, 0];
        vm.step();
        assert_eq!(vm.equal_flag, true);
        vm.registers[1] = 20;
        vm.step();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut vm = VM::new();
        vm.registers[0] = 7;
        vm.equal_flag = true;
        vm.program = vec![Opcode::JEQ.into(), 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        vm.step();
        assert_eq!(vm.pc, 7);
    }

    #[test]
    fn test_opcode_jneq() {
        let mut vm = VM::new();
        vm.registers[0] = 6;
        vm.equal_flag = true;
        vm.program = vec![Opcode::JNEQ.into(), 0, Opcode::JNEQ.into(), 0, 0, 0, 0, 0];
        vm.step();
        assert_eq!(vm.pc, 2);
        vm.equal_flag = false;
        vm.step();
        assert_eq!(vm.pc, 6);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = VM::new();
        let test_bytes = vec![Opcode::HLT.into(), 0, 0, 0];
        vm.program = test_bytes;
        vm.step();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        vm.program = test_bytes;
        vm.step();
        assert_eq!(vm.pc, 1);
    }
}
