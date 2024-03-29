use crate::assembler::token::Token;
use crate::instruction::Opcode;
use nom::character::complete::alpha1;
use nom::{do_parse, named};

named!(
    pub opcode<&str, Token>,
    do_parse!(
        op: alpha1 >>
        (
            Token::Op { code: Opcode::from(op) }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode() {
        let result = opcode("load");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let (rest, token) = opcode("daol").unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
        assert_eq!(rest, "");
    }
}
