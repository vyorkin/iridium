use crate::assembler::{
    parsing::{comment::comment, label::label_decl, opcode, operand::operand, ParsingError},
    token::Token,
};

use nom::{
    character::complete::{multispace0, space1},
    do_parse, named, opt, preceded,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    opcode: Token,
    label: Option<Token>,
    directive: Option<Token>,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl Instruction {
    pub fn to_bytes(&self) -> Result<Vec<u8>, ParsingError> {
        let mut bytes = vec![];
        let opcode = self
            .opcode_bytes()
            .ok_or_else(|| ParsingError::OpcodeExpected(self.opcode.clone()))?;
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
        for op in operands.into_iter().flatten() {
            bytes.append(&mut op.operand_bytes().unwrap())
        }
        bytes
    }
}

named!(
    pub instruction<&str, Instruction>,
    do_parse!(
    multispace0 >>
    opt!(comment) >>
    label: opt!(label_decl) >>
    opcode: opcode >>
    operand1: opt!(preceded!(space1, operand)) >>
    operand2: opt!(preceded!(space1, operand)) >>
    operand3: opt!(preceded!(space1, operand)) >>
    opt!(comment) >>
    multispace0 >>
    (
        Instruction {
            opcode,
            label,
            directive: None,
            operand1,
            operand2,
            operand3
        }
    )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_nullary() {
        let actual = instruction("hlt\n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::HLT },
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        };
        assert_eq!(Ok(("", expected)), actual);
    }

    #[test]
    fn test_parse_instruction_binary() {
        let actual = instruction("load $0 #100  \n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::LOAD },
            label: None,
            directive: None,
            operand1: Some(Token::Register { reg_num: 0 }),
            operand2: Some(Token::Number { value: 100 }),
            operand3: None,
        };
        assert_eq!(Ok(("", expected)), actual);
    }

    #[test]
    fn test_parse_instruction_ternary() {
        let actual = instruction("add $0 $1 $2\n\n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::ADD },
            label: None,
            directive: None,
            operand1: Some(Token::Register { reg_num: 0 }),
            operand2: Some(Token::Register { reg_num: 1 }),
            operand3: Some(Token::Register { reg_num: 2 }),
        };
        assert_eq!(Ok(("", expected)), actual);
    }

    #[test]
    fn test_parse_with_comment_preceded() {
        let actual = instruction("; whatever comment   \n  mul #100 $2 $3\n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::MUL },
            label: None,
            directive: None,
            operand1: Some(Token::Number { value: 100 }),
            operand2: Some(Token::Register { reg_num: 2 }),
            operand3: Some(Token::Register { reg_num: 3 }),
        };
        assert_eq!(Ok(("", expected)), actual);
    }

    #[test]
    fn test_parse_with_comment_terminated() {
        let actual = instruction("jmpf $2    ; comment text\n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::JMPF },
            label: None,
            directive: None,
            operand1: Some(Token::Register { reg_num: 2 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(Ok(("", expected)), actual);
    }
}
