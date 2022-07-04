use nom::{
    character::complete::{multispace0, space0},
    do_parse, named, opt, tag, take_until,
};

use crate::assembler::token::Token;

named!(
    pub comment<&str, Token>,
    do_parse!(
        space0 >>
        tag!(";") >>
        take_until!("\n") >>
        multispace0 >>
        (
            Token::Comment
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comment() {
        let actual = comment("; blah blah\n   \t\n123");
        assert_eq!(Ok(("123", Token::Comment)), actual);
    }
}
