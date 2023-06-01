use nom::{
    IResult,
    branch::alt,
    bytes::complete::take_while1,
    Parser,
    sequence::preceded,
    character::complete::{char, space0}};

use super::{_Token, is_special};

#[cfg(feature = "std")]
use nom::multi::{many0, many1};


fn parse_token(input: &str) -> IResult<&str, _Token> {
    preceded(space0,
    alt((
        char('.').map(|_| _Token::_Period),
        char('/').map(|_| _Token::_ForwardSlash),
        char('(').map(|_| _Token::_LeftParenthesis),
        char(')').map(|_| _Token::_RightParenthesis),
        char('[').map(|_| _Token::_LeftBracket),
        char(']').map(|_| _Token::_RightBracket),
        char('{').map(|_| _Token::_LeftBrace),
        char('}').map(|_| _Token::_RightBrace),
        char(',').map(|_| _Token::_Comma),
        char(';').map(|_| _Token::_Semicolon),
        char(':').map(|_| _Token::_Colon),
        char('%').map(|_| _Token::_Percent),
        take_while1(|c: char| !c.is_whitespace() && !is_special(c)).map(_Token::_String),
    )))(input)
}

#[cfg(feature = "std")]
fn parse_line(input: &str) -> IResult<&str, Vec<_Token>> {
    many1(parse_token)(input.trim_start())
}

#[cfg(feature = "std")]
fn parse_file(input: &str) -> IResult<&str, Vec<Vec<_Token>>> {
    many0(parse_line)(input)
}

#[cfg(test)]
mod token_tests {
    #[cfg(feature = "std")]
    use crate::ptx_files::_EXAMPLE_FILE;

    use super::*;

    #[test]
    fn test_parse_token_period() {
        assert_eq!(parse_token("."), Ok(("", _Token::_Period)));
    }

    #[test]
    fn test_parse_token_backslash() {
        assert_eq!(parse_token("/"), Ok(("", _Token::_ForwardSlash)));
    }

    #[test]
    fn test_parse_token_string() {
        assert_eq!(
            parse_token("Hello, world! "),
            Ok((", world! ", _Token::_String("Hello")))
        );
    }

    #[test]
    fn test_parse_token_empty() {
        assert!(parse_token("").is_err());
    }

    #[test]
    fn test_parse_token_whitespace() {
        assert!(parse_token(" ").is_err());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_tokens() {
        assert_eq!(
            parse_line(".version 7.5"),
            Ok((
                "",
                vec![
                    _Token::_Period,
                    _Token::_String("version"),
                    _Token::_String("7"),
                    _Token::_Period,
                    _Token::_String("5"),
                ]
            ))
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_parse_line() {
        match parse_line(_EXAMPLE_FILE) {
            Ok((input, tokens)) => {
                tokens.iter().for_each(|token|
                    println!("{token:?}")
                );
                println!("{input}");
            },
            Err(e) => {
                println!("{e:?}")
            }
        }   
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_parse_file() {
        match parse_file(_EXAMPLE_FILE) {
            Ok((input, tokens)) => {
                tokens.iter().for_each(|token|
                    println!("{token:?}")
                );
                println!("{input}");
            },
            Err(e) => {
                println!("{e:?}")
            }
        }
    }
}