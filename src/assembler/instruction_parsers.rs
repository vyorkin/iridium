use nom::character::complete::{line_ending, space1};
use nom::{do_parse, named, opt};
use std::error::Error;
use std::fmt::{self, Display};

use crate::assembler::opcode::Token;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::number;
use crate::assembler::register_parsers::register;

#[derive(Debug, Clone)]
pub enum InstructionError {
    NonOpcode(Token),
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionError::NonOpcode(token) => {
                write!(f, "Non-opcode found in opcode field: {}", token)
            }
        }
    }
}

impl Error for InstructionError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl Instruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>, InstructionError> {
        let mut bytes = vec![];
        let opcode = self
            .opcode_bytes()
            .ok_or(InstructionError::NonOpcode(self.opcode.clone()))?;
        let mut operands = self.operand_bytes();
        bytes.push(opcode);
        bytes.append(&mut operands);
        Ok(bytes)
    }

    fn opcode_bytes(&self) -> Option<u8> {
        if let Token::Op { code } = &self.opcode {
            Some(code.clone().into())
        } else {
            None
        }
    }

    fn operand_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let operands = vec![&self.operand1, &self.operand2, &self.operand3];
        for op in operands {
            if let Some(token) = op {
                bytes.append(&mut token.operand_bytes().unwrap())
            }
        }
        bytes
    }
}

named!(
    pub instruction_one<&str, Instruction>,
    do_parse!(
        opcode: opcode_load >>
        space1 >>
        reg: register >>
        space1 >>
        operand: number >>
        opt!(line_ending) >>
        (
            Instruction {
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
        let result = instruction_one("load $0 #100\n");
        let instruction = Instruction {
            opcode: Token::Op { code: Opcode::LOAD },
            operand1: Some(Token::Register { reg_num: 0 }),
            operand2: Some(Token::Number { value: 100 }),
            operand3: None,
        };
        assert_eq!(Ok(("", instruction)), result);
    }
}
