use nom::{do_parse, named, tag};
use nom::character::complete::digit1;
use crate::assembler::opcode::Token;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = number("#10");
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(value, Token::Number { value: 10 });

        let result = number("10");
        assert_eq!(result.is_ok(), false);
    }
}
