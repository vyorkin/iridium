use std::error::Error;
use std::fmt::{self, Display};
use crate::assembler::opcode::Token;

#[derive(Debug, Clone)]
pub enum ParsingError {
    OpcodeExpected(Token),
    UnknownOpcode(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::OpcodeExpected(token) => {
                write!(f, "Non-opcode found in opcode field: {}", token)
            }
            ParsingError::UnknownOpcode(s) => {
                write!(f, "Unknown opcode: {}", s)
            }
        }
    }
}

impl Error for ParsingError {}
