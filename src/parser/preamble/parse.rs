use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, space1},
    combinator::opt,
    sequence::{preceded, Tuple},
    IResult, Parser,
};

use crate::parser::{comment::parse::many1_comments_or_whitespace, parse_name};

use super::{Preamble, Target, Version, AddressSize};

pub(crate) fn parse_preamble(input: &str) -> IResult<&str, Preamble> {
    (
        preceded(
            opt(many1_comments_or_whitespace), 
            parse_version
        ),
        preceded(
            many1_comments_or_whitespace,
            parse_target
        ),
        preceded(
            many1_comments_or_whitespace,
            parse_address_size
        )
    )
    .parse(input)
    .map(|(input, (version, target, address_size))| {
        (
            input,
            Preamble {
                version,
                target,
                address_size,
            },
        )
    })
}

pub(super) fn parse_version(input: &str) -> IResult<&str, Version> {
    (
        preceded(tag(".version").and(space1), take_while1(char::is_numeric)),
        preceded(char('.'), take_while1(char::is_numeric)),
    )
    .parse(input)
    .map(|(input, (major, minor))| (input, Version { major, minor }))
}

pub(super) fn parse_target(input: &str) -> IResult<&str, Target> {
    preceded(
        tag(".target").and(space1),
        parse_name.map(|target| Target { target }),
    )(input)
}

pub(super) fn parse_address_size(input: &str) -> IResult<&str, AddressSize> {
    preceded(
        tag(".address_size").and(space1),
        parse_name.map(|size| AddressSize { size }),
    )(input)
}
