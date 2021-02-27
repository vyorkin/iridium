use nom::{do_parse, named, separated_list1};
use nom::character::complete::line_ending;

use crate::assembler::instruction_parsers::{Instruction, instruction_one};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instr in &self.instructions {
            program.append(&mut instr.to_bytes().unwrap());
        }
        program
    }
}

named!(
    pub program<&str, Program>,
    do_parse!(
        instructions: separated_list1!(line_ending, instruction_one) >>
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
        assert_eq!(result.is_ok(), true);
        let (rest, p) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(1, p.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
    }
}
