use nom::character::complete::line_ending;
use nom::{do_parse, named, separated_list1};

use crate::assembler::parsing::{instruction, Instruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for instr in &self.instructions {
            bytes.append(&mut instr.to_bytes().unwrap());
        }
        bytes
    }
}

named!(
    pub program<&str, Program>,
    do_parse!(
        instructions: separated_list1!(line_ending, instruction) >>
        (
            Program { instructions }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program("load $0 #100\n");
        assert!(result.is_ok());
        let (rest, p) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(1, p.instructions.len());
        assert_eq!(p.to_bytes().len(), 4);
    }

    #[test]
    fn test_parse_multiline_program() {
        let code = "
            load $0 #50 ; load number 50 into reg 0
            add $0 #10  ; add 10 to reg 0
        ";
        let result = program(code);
        // assert!(result.is_ok());
        // let (rest, p) = result.unwrap();
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100\n");
        assert!(result.is_ok());
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
    }
}
