use nom::{
    IResult,
    sequence::delimited,
    bytes::complete::{tag, take_while1},
    character::complete::{char, space1}, Parser,
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

#[cfg(test)]
mod test_parse_global {
    use crate::parser::Global;

    use super::parse_global;

    #[test]
    fn trivial_exaample() {
        let input = ".global hello;";
        let expected = Ok(("", Global { raw_string: "hello" }));
        assert_eq!(parse_global(input), expected)
    }
}