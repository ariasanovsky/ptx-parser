use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while1},
    Parser,
    multi::{many0, many1},
    sequence::{preceded, delimited},
    character::complete::{char, space0}};

use super::Version;

fn parse_version(input: &str) -> IResult<&str, Version> {
    preceded(
        space0.and(tag(".version")),
        take_while1(|c: char| c.is_numeric())
        .and(
            preceded(
                char('.'),
                take_while1(|c: char| c.is_numeric())
    )))(input).map(|(rest, (major, minor) )| (rest, Version { major, minor }))
}

