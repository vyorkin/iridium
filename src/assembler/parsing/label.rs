use nom::{character::complete::alphanumeric1, do_parse, named, tag};

use crate::assembler::token::Token;

named!(
    pub label_decl<&str, Token>,
    do_parse!(
        name: alphanumeric1 >>
        tag!(":") >>
        (
            Token::LabelDecl { name: name.to_string() }
        )
    )
);

named!(
    pub label_usage<&str, Token>,
    do_parse!(
        tag!("@") >>
        name: alphanumeric1 >>
        (
            Token::LabelUsage { name: name.to_string() }
        )
    )
);

#[cfg(test)]
mod tests {
    use crate::assembler::{parsing::label::label_usage, token::Token};

    use super::label_decl;

    #[test]
    fn test_parse_label_decl() {
        let result = label_decl("foo:");
        assert!(result.is_ok());
        let (rest, actual) = result.unwrap();
        let expected = Token::LabelDecl {
            name: "foo".to_string(),
        };
        assert_eq!(actual, expected);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage("@bar");
        assert!(result.is_ok());
        let (rest, actual) = result.unwrap();
        let expected = Token::LabelUsage {
            name: "bar".to_string(),
        };
        assert_eq!(actual, expected);
        assert_eq!(rest, "");
    }
}
