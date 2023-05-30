use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_until},
    Parser,
    sequence::{preceded, delimited},
    character::complete::{char, space0, multispace1}, combinator::{opt, map_res}, multi::many0_count};

use super::Comment;

pub(crate) fn empty_comments_and_whitespace(input: &str) -> IResult<&str, usize> {
    many0_count(
        empty_comment_or_whitespace
    )(input)
}

pub(crate) fn empty_comment_or_whitespace(input: &str) -> IResult<&str, ()> {
    alt((
        multispace1
        .map(|_| ()),
        parse_line_comment
        .map(|_| ()),
    ))(input)
}

pub(crate) fn parse_line_comment(input: &str) -> IResult<&str, Comment> {
    preceded(
        char('/'),
        alt((
            preceded(
                char('/'),
                take_while(|c: char| c != '\n')
            )
            .map(Comment::Line),
            delimited(
                char('*'),
                take_until("*/"),
                tag("*/")
            )
            .map(Comment::Block)
    )))(input)
}

#[cfg(test)]
mod test_parse_line_comment {
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

#[cfg(test)]
mod test_empty_comments_and_whitespace {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(
            empty_comments_and_whitespace(""),
            Ok(("", 0))
        );
    }
    
    #[test]
    fn test_new_line() {
        assert_eq!(
            empty_comments_and_whitespace("\n"),
            Ok(("", 1))
        );
    }

    #[test]
    fn test_empty_comments_and_whitespace() {
        assert_eq!(
            empty_comments_and_whitespace("  // This is a comment\n  // with another line\n"),
            Ok(("", 5))
        );
    }

    #[test]
    fn test_empty_comments_and_whitespace_with_leading_whitespace() {
        assert_eq!(
            empty_comments_and_whitespace("  // This is a comment\n  // with another line\n"),
            Ok(("", 5))
        );
    }
}