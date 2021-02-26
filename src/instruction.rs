/// VM opcodes.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    /// No operation.
    NOP,
    /// Load a number into register.
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    /// Equality comparison.
    EQ,
    /// Abolute jump.
    JMP,
    /// Forward relative jump.
    JMPF,
    /// Backward relative jump.
    JMPB,
    /// Halt VM execution.
    HLT,
    /// Illegal opcode encountered.
    IGL(u8),
}

impl From<u8> for Opcode {
    fn from(source: u8) -> Self {
        match source {
            0 => Opcode::NOP,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            99 => Opcode::HLT,
            b => Opcode::IGL(b),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(source: Opcode) -> Self {
        match source {
            Opcode::NOP => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MUL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::HLT => 99,
            Opcode::IGL(b) => b,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
