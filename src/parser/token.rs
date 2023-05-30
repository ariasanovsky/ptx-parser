use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    Parser,
    multi::{many0, many1},
    sequence::preceded,
    character::complete::{char, space0}};

use super::{Token, is_special};

fn parse_token(input: &str) -> IResult<&str, Token> {
    preceded(space0,
    alt((
        char('.').map(|_| Token::Period),
        char('/').map(|_| Token::ForwardSlash),
        char('(').map(|_| Token::LeftParenthesis),
        char(')').map(|_| Token::RightParenthesis),
        char('[').map(|_| Token::LeftBracket),
        char(']').map(|_| Token::RightBracket),
        char('{').map(|_| Token::LeftBrace),
        char('}').map(|_| Token::RightBrace),
        char(',').map(|_| Token::Comma),
        char(';').map(|_| Token::Semicolon),
        char(':').map(|_| Token::Colon),
        char('%').map(|_| Token::Percent),
        take_while1(|c: char| !c.is_whitespace() && !is_special(c)).map(Token::String),
    )))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Token>> {
    many1(parse_token)(input.trim_start())
}

fn parse_file(input: &str) -> IResult<&str, Vec<Vec<Token>>> {
    many0(parse_line)(input)
}

#[cfg(test)]
mod token_tests {
    use crate::parser::_EXAMPLE_FILE;

    use super::*;

    #[test]
    fn test_parse_token_period() {
        assert_eq!(parse_token("."), Ok(("", Token::Period)));
    }

    #[test]
    fn test_parse_token_backslash() {
        assert_eq!(parse_token("/"), Ok(("", Token::ForwardSlash)));
    }

    #[test]
    fn test_parse_token_string() {
        assert_eq!(
            parse_token("Hello, world! "),
            Ok((", world! ", Token::String("Hello")))
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

    #[test]
    fn test_tokens() {
        assert_eq!(
            parse_line(".version 7.5"),
            Ok((
                "",
                vec![
                    Token::Period,
                    Token::String("version"),
                    Token::String("7"),
                    Token::Period,
                    Token::String("5"),
                ]
            ))
        );
    }

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