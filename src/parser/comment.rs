use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_until},
    Parser,
    sequence::{preceded, delimited},
    character::complete::{char, space0}, combinator::opt};

use super::Comment;

pub(crate) fn parse_line_comment(input: &str) -> IResult<&str, Comment> {
    preceded(
        char('/'),
        alt((
            preceded(
                char('/'),
                take_while(|c: char| c != '\n')
            )
            .map(|comment| Comment::Line(comment)),
            delimited(
                char('*'),
                take_until("*/"),
                tag("*/")
            )
            .map(|comment| Comment::Block(comment))
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_comment_single_line() {
        assert_eq!(
            parse_line_comment("// This is a comment\n"),
            Ok(("\n", Comment::Line(" This is a comment")))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_with_trailing_whitespace() {
        assert_eq!(
            parse_line_comment("// This is another comment with trailing whitespace    \n"),
            Ok(("\n", Comment::Line(" This is another comment with trailing whitespace    ")))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_empty() {
        assert_eq!(
            parse_line_comment("//\n"),
            Ok(("\n", Comment::Line("" )))
        );
    }

    #[test]
    fn test_parse_line_comment_single_line_with_leading_whitespace() {
        assert!(
            parse_line_comment("  // This is a comment with leading whitespace\n")
            .is_err()
        );
    }

    #[test]
    fn test_parse_line_comment_multi_line() {
        assert_eq!(
            parse_line_comment("// This is a comment that extends over multiple lines\n// with another line\n"),
            Ok(("\n// with another line\n", Comment::Line(" This is a comment that extends over multiple lines")))
        );
    }

    #[test]
    fn test_parse_line_comment_block_comment() {
        assert_eq!(
            parse_line_comment("/* This is a block comment\n and it's on \n 3 lines */\n"),
            Ok(("\n", Comment::Block(" This is a block comment\n and it's on \n 3 lines ")))
        );
    }
}