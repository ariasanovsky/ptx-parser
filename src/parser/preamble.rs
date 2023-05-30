use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while1},
    Parser,
    multi::{many0, many1},
    sequence::{preceded, delimited},
    character::complete::{char, space0, space1}};

use super::Version;

fn parse_version(input: &str) -> IResult<&str, Version> {
    preceded(
        space0.and(tag(".version")).and(space1),
        take_while1(|c: char| c.is_numeric())
        .and(
            preceded(
                char('.'),
                take_while1(|c: char| c.is_numeric())
    )))(input).map(|(rest, (major, minor) )| 
        (rest, Version { major, minor })
    )
}

#[cfg(test)]
mod test {
    use crate::parser::Version;
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(
            parse_version(".version 1.0"),
            Ok(("", Version { major: "1", minor: "0" }))
        );
    }

    #[test]
    fn test_parse_version_with_leading_whitespace() {
        assert_eq!(
            parse_version("  .version 1.0"),
            Ok(("", Version { major: "1", minor: "0" }))
        );
    }

    #[test]
    fn test_parse_version_with_trailing_whitespace() {
        assert_eq!(
            parse_version(".version 1.0  "),
            Ok(("  ", Version { major: "1", minor: "0" }))
        );
    }

    #[test]
    fn test_parse_version_with_comment() {
        assert_eq!(
            parse_version(".version 1.0// This is a comment"),
            Ok(("// This is a comment", Version { major: "1", minor: "0" }))
        );
    }
}