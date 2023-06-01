use nom::{
    bytes::complete::take_while1,
    IResult,
    sequence::delimited,
    character::complete::char,
};

pub(crate) mod token;
pub(crate) mod preamble;
pub(crate) mod comment;
pub(crate) mod function;

#[derive(Debug, PartialEq)]
enum _Token<'a> {
    _Period,
    _ForwardSlash,
    _String(&'a str),
    _LeftParenthesis,
    _RightParenthesis,
    _LeftBracket,
    _RightBracket,
    _LeftBrace,
    _RightBrace,
    _Comma,
    _Semicolon,
    _Colon,
    _Percent,
}


#[derive(Debug, PartialEq)]
pub(crate) enum Comment<'a> {
    Line(&'a str),
    Block(&'a str),
}

#[derive(Debug, PartialEq)]
struct Preamble<'a> {
    version: preamble::Version<'a>,
    target: preamble::Target<'a>,
    address_size: preamble::AddressSize<'a>,
}

#[derive(Debug, PartialEq)]
struct Function<'a> {
    signature: function::FunctionSignature<'a>,
    body: Option<function::FunctionBody<'a>>,
}

fn is_special(c: char) -> bool {
    ['.', '/', '(', ')', '[', ']', '{', '}', ',', ';', ':', '%'].contains(&c)
}

pub(crate) fn parse_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && !is_special(c))(input)
}

pub(crate) fn parse_parenthesized_naive(input: &str) -> IResult<&str, &str> {
    delimited(
        char('('),
        take_while1(|c: char| c != ')'),
        char(')')
    )(input)
}

#[cfg(test)]
mod test_parse_parenthesized {
    
    use super::parse_parenthesized_naive;
    
    #[test]
    fn no_newline() {
        let input = "(hello)";
        let expected = Ok(("", "hello"));
        assert_eq!(parse_parenthesized_naive(input), expected)
    }

    #[test]
    fn newline() {
        let input = "(hello\n)";
        let expected = Ok(("", "hello\n"));
        assert_eq!(parse_parenthesized_naive(input), expected)
    }

    #[test]
    fn one_left_parenthesis() {
        let input = "(hello";
        assert!(
            parse_parenthesized_naive(input).is_err()
        )
    }

    #[test]
    fn two_left_one_right() {
        let input = "((hello)";
        assert_eq!(
            parse_parenthesized_naive(input),
            Ok(("", "(hello")),
        )
    }
}