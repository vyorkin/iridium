use crate::instruction::Opcode;

/// Virtual machine state.
pub struct VM {
    /// VM registers.
    registers: [i32; 32],
    /// Program counter.
    pc: usize,
    /// Contains program bytecode.
    program: Vec<u8>,
    /// Stores a division remainder.
    remainder: u32,
}

impl VM {
    /// Initializes a fresh VM state.
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
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
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::NOP => {}
            Opcode::LOAD => {
                let reg = self.next_8_bits() as usize;
                let num = self.next_16_bits() as u16;
                self.registers[reg] = num as i32;
            }
            Opcode::ADD => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 + reg2;
            }
            Opcode::SUB => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 - reg2;
            }
            Opcode::MUL => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 * reg2;
            }
            Opcode::DIV => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 + reg2;
                self.remainder = (reg1 % reg2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::HLT => {
                println!("HLT encountered, stopping VM");
                return true;
            }
            Opcode::IGL(b) => {
                println!("Unknown opcode {}, terminating VM", b);
                return true;
            }
        }
        false
    }

    /// Reads next byte as `u8`.
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    /// Reads next 2 bytes as `u16`.
    fn next_16_bits(&mut self) -> u16 {
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
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.program = vec![0, 0, Opcode::JMP.into(), 0, 0];
        vm.step_times(3);
        assert_eq!(vm.pc, 1);
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
