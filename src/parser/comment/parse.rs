use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, multispace1},
    multi::many1_count,
    sequence::{delimited, preceded},
    IResult, Parser,
};

use super::Comment;

pub(crate) fn many1_comments_or_whitespace(input: &str) -> IResult<&str, usize> {
    many1_count(comment_or_whitespace)(input)
}

pub(crate) fn comment_or_whitespace(input: &str) -> IResult<&str, &str> {
    alt((
        multispace1,
        parse_line_comment
        .map(|comment| match comment {
            Comment::Line(comment) => comment,
            Comment::Block(comment) => comment,
        }),
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
