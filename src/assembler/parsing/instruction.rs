use crate::assembler::{
    opcode::Token,
    parsing::{number, opcode, register, ParsingError},
};
use nom::character::complete::{multispace0, space1};
use nom::{do_parse, named, opt, alt};

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    opcode: Token,
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
    pub instruction_0<&str, Instruction>,
    do_parse!(
        opcode: opcode >>
        multispace0 >>
        (
            Instruction {
                opcode,
                operand1: None,
                operand2: None,
                operand3: None,
            }
        )
    )
);

named!(
    pub instruction_2<&str, Instruction>,
    do_parse!(
        opcode: opcode >>
        space1 >>
        reg: opt!(register) >>
        space1 >>
        operand: opt!(number) >>
        multispace0 >>
        (
            Instruction {
                opcode,
                operand1: reg,
                operand2: operand,
                operand3: None,
            }
        )
    )
);

named!(
    pub instruction<&str, Instruction>,
    do_parse!(
        ins: alt!(instruction_2 | instruction_0) >> (ins)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_nullary() {
        let actual = instruction_0("hlt\n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::HLT },
            operand1: None,
            operand2: None,
            operand3: None,
        };
        assert_eq!(Ok(("", expected)), actual);
    }

    #[test]
    fn test_parse_instruction_binary() {
        let actual = instruction_2("load $0 #100  \n");
        let expected = Instruction {
            opcode: Token::Op { code: Opcode::LOAD },
            operand1: Some(Token::Register { reg_num: 0 }),
            operand2: Some(Token::Number { value: 100 }),
            operand3: None,
        };
        assert_eq!(Ok(("", expected)), actual);
    }
}
