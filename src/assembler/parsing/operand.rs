use crate::assembler::token::Token;
use nom::character::complete::digit1;
use nom::{alt, do_parse, named, tag};

use super::register;

named!(
    pub number<&str, Token>,
    do_parse!(
        tag!("#") >>
        int_num: digit1 >>
        (
            Token::Number { value: int_num.parse::<i32>().unwrap() }
        )
    )
);

named!(
    pub operand<&str, Token>,
    alt!(number | register)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let reg_result = operand("$0");
        let num_result = operand("#100");

        assert!(reg_result.is_ok());
        assert!(num_result.is_ok());
    }

    #[test]
    fn test_parse_number() {
        let result = number("#10");
        assert!(result.is_ok());
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Number { value: 10 });

        let result = number("10");
        assert!(result.is_err());
    }
}
