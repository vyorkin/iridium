use crate::assembler::opcode::Token;
use nom::character::complete::digit1;
use nom::{do_parse, named, tag};

named!(
    pub register<&str, Token>,
    do_parse!(
        tag!("$") >>
        reg_num: digit1 >>
        (
            Token::Register {
                reg_num: reg_num.parse::<u8>().unwrap()
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register("$0");
        assert_eq!(result.is_ok(), true);
        let result = register("0");
        assert_eq!(result.is_ok(), false);
        let result = register("$a");
        assert_eq!(result.is_ok(), false);
    }
}
