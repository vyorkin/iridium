use nom::{do_parse, named};
use nom::character::complete::{space0, space1, newline};

use crate::assembler::opcode::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::register_parsers::register;
use crate::assembler::operand_parsers::number;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

named!(
    pub instruction_one<&str, AssemblerInstruction>,
    do_parse!(
        opcode: opcode_load >>
        space1 >>
        reg: register >>
        space1 >>
        operand: number >>
        newline >>
        (
            AssemblerInstruction {
                opcode,
                operand1: Some(reg),
                operand2: Some(operand),
                operand3: None,
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_from_one() {
        let result = instruction_one(&"load $0 #100\n");
        let instruction = AssemblerInstruction {
            opcode: Token::Op { code: Opcode::LOAD },
            operand1: Some(Token::Register { reg_num: 0 }),
            operand2: Some(Token::Number { value: 100 }),
            operand3: None,
        };
        assert_eq!(Ok(("", instruction)), result);
    }
}
