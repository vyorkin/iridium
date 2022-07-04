use nom::{character::complete::multispace0, do_parse, named, tag, take_until};

use crate::assembler::token::Token;

named!(
    pub directive<&str, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive { name: name.to_string() }
        )
    )
);
