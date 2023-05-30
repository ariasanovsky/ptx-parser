use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    Parser,
    multi::{many0, many1},
    sequence::{preceded, delimited},
    character::complete::{char, space0}, combinator::opt};

use super::LineComment;

pub(crate) fn parse_line_comment(input: &str) -> IResult<&str, LineComment> {
    preceded(
        space0,
        delimited(
            tag("//"),
            take_while(|c: char| c != '\n'),
            opt(char('\n'))
    ))(input).map(|(rest, text)| (rest, LineComment { text }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_comment_single_line() {
        assert_eq!(
            parse_line_comment("// This is a comment\n"),
            Ok(("", LineComment { text: " This is a comment"} ))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_with_trailing_whitespace() {
        assert_eq!(
            parse_line_comment("// This is another comment with trailing whitespace    \n"),
            Ok(("", LineComment { text: " This is another comment with trailing whitespace    "} ))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_empty() {
        assert_eq!(
            parse_line_comment("//\n"),
            Ok(("", LineComment { text: "" } ))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_with_leading_whitespace() {
        assert_eq!(
            parse_line_comment("  // This is a comment with leading whitespace\n"),
            Ok(("", LineComment { text: " This is a comment with leading whitespace" } ))
        );
    }

    #[test]
    fn test_parse_line_comment_multi_line() {
        assert_eq!(
            parse_line_comment("// This is a comment that extends over multiple lines\n// with another line\n"),
            Ok(("// with another line\n", LineComment { text: " This is a comment that extends over multiple lines" } ))
        );
    }

    #[test]
    fn test_parse_line_comment_block_comment() {
        assert!(
            parse_line_comment("/* This is a block comment */\n").is_err()
        );
    }
}