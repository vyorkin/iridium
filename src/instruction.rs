/// VM opcodes.
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    /// No operation.
    NOP,
    /// Load a number into register.
    LOAD,
    /// Allocate a chunk of memory from a heap.
    ALLOC,
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
    /// Increment.
    INC,
    /// Decrement.
    DEC,
    /// Halt VM execution.
    HLT,
    /// Illegal opcode encountered.
    IGL,
}

impl From<&str> for Opcode {
    fn from(source: &str) -> Self {
        match source {
            "nop" => Opcode::NOP,
            "load" => Opcode::LOAD,
            "alloc" => Opcode::ALLOC,
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
            "inc" => Opcode::INC,
            "dec" => Opcode::DEC,
            "hlt" => Opcode::HLT,
            _ => Opcode::IGL,
        }
    }
}

impl From<u8> for Opcode {
    fn from(source: u8) -> Self {
        match source {
            0 => Opcode::NOP,
            1 => Opcode::LOAD,
            2 => Opcode::ALLOC,
            3 => Opcode::ADD,
            4 => Opcode::SUB,
            5 => Opcode::MUL,
            6 => Opcode::DIV,
            7 => Opcode::JMP,
            8 => Opcode::JMPF,
            9 => Opcode::JMPB,
            10 => Opcode::EQ,
            11 => Opcode::JEQ,
            12 => Opcode::JNEQ,
            13 => Opcode::INC,
            14 => Opcode::DEC,
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
            Opcode::ALLOC => 2,
            Opcode::ADD => 3,
            Opcode::SUB => 4,
            Opcode::MUL => 5,
            Opcode::DIV => 6,
            Opcode::JMP => 7,
            Opcode::JMPF => 8,
            Opcode::JMPB => 9,
            Opcode::EQ => 10,
            Opcode::JEQ => 11,
            Opcode::JNEQ => 12,
            Opcode::INC => 13,
            Opcode::DEC => 14,
            Opcode::HLT => 99,
            Opcode::IGL => 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_str() {
        let opcode = Opcode::from("load");
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from("illegal");
        assert_eq!(opcode, Opcode::IGL);
    }

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }
}
