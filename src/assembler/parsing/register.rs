use crate::assembler::token::Token;
use nom::{character::complete::digit1, do_parse, named, tag};

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
        assert!(result.is_ok());
        let result = register("0");
        assert!(result.is_err());
        let result = register("$a");
        assert!(result.is_err());
    }
}
