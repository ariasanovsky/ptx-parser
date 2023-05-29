use nom::{IResult, branch::alt, bytes::complete::{tag, take_while, take_while1}, Parser, multi::{many0, many1}, sequence::preceded, character::complete::space0};

use super::Token;

fn parse_token(input: &str) -> IResult<&str, Token> {
    preceded(space0,
    alt((
        tag(".").map(|_| Token::Period),
        tag("/").map(|_| Token::ForwardSlash),
        tag("(").map(|_| Token::LeftParenthesis),
        tag(")").map(|_| Token::RightParenthesis),
        tag("[").map(|_| Token::LeftBracket),
        tag("]").map(|_| Token::RightBracket),
        tag("{").map(|_| Token::LeftBrace),
        tag("}").map(|_| Token::RightBrace),
        tag(",").map(|_| Token::Comma),
        tag(";").map(|_| Token::Semicolon),
        tag(":").map(|_| Token::Colon),
        tag("%").map(|_| Token::Percent),
        take_while1(|c: char| !c.is_whitespace() && !is_special(c)).map(Token::String),
    )))(input)
}

fn is_special(c: char) -> bool {
    ['.', '/', '(', ')', '[', ']', '{', '}', ',', ';', ':', '%'].contains(&c)
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