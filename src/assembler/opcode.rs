use std::{convert::TryInto, error::Error};
use std::fmt::{self, Display};
use crate::instruction::Opcode;

#[derive(Debug, Clone)]
pub enum TokenError {
    UnexpectedOpcode(Token),
}

impl Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenError::UnexpectedOpcode(token) => {
                write!(f, "Unexpected opcode found in operand field: {}", token)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    Number { value: i32 },
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Op { code } => write!(f, "{:?}", code),
            Token::Register { reg_num } => write!(f, "${0}", reg_num),
            Token::Number { value } => write!(f, "{}", value),
        }
    }
}

impl Token {
    pub fn operand_bytes(&self) -> Result<Vec<u8>, TokenError> {
        let mut bytes = vec![];
        match self {
            Token::Register { reg_num } => {
                bytes.push(*reg_num);
            }
            Token::Number { value } => {
                let converted: u16 = *value as u16;
                let hi = converted;
                let lo = hi >> 8;
                bytes.push(lo as u8);
                bytes.push(hi as u8);
            }
            token => {
                return Err(TokenError::UnexpectedOpcode(token.clone()));
            },
        }
        Ok(bytes)
    }
}
