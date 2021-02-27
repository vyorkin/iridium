use nom::{do_parse, named, tag_no_case};
use crate::assembler::opcode::Token;
use crate::instruction::Opcode;

named!(
    pub opcode_load<&str, Token>,
    do_parse!(
        tag_no_case!("load") >> (Token::Op { code: Opcode::LOAD })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode_load("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_load("daol");
        assert_eq!(result.is_ok(), false);
    }
}
