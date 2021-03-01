use crate::assembler::parsing::ParsingError;
use std::str::FromStr;

/// VM opcodes.
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    /// No operation.
    NOP,
    /// Load a number into register.
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    /// Abolute jump.
    JMP,
    /// Forward relative jump.
    JMPF,
    /// Backward relative jump.
    JMPB,
    /// Equality comparison.
    EQ,
    /// Jump if equal.
    JEQ,
    /// Jump if not equal.
    JNEQ,
    /// Halt VM execution.
    HLT,
    /// Illegal opcode encountered.
    IGL,
}

impl FromStr for Opcode {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opcode = match s {
            "nop" => Opcode::NOP,
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
            "jmpb" => Opcode::JMPB,
            "eq" => Opcode::EQ,
            "jeq" => Opcode::JEQ,
            "jneq" => Opcode::JNEQ,
            "hlt" => Opcode::HLT,
            _ => Opcode::IGL,
        };
        Ok(opcode)
    }
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
            10 => Opcode::JEQ,
            11 => Opcode::JNEQ,
            99 => Opcode::HLT,
            _ => Opcode::IGL,
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
            Opcode::JEQ => 10,
            Opcode::JNEQ => 11,
            Opcode::HLT => 99,
            Opcode::IGL => 100,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let opcode = Opcode::from_str("load").unwrap();
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from_str("illegal").unwrap();
        assert_eq!(opcode, Opcode::IGL);
    }

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
