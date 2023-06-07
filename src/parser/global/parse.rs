use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, space1},
    sequence::delimited,
    IResult, Parser,
};

use super::Global;

pub(crate) fn parse_global(input: &str) -> IResult<&str, Global> {
    delimited(
        tag(".global").and(space1),
        take_while1(|c: char| c != ';')
        .map(|raw_string| Global { raw_string }),
        char(';'),
    )(input)
}
