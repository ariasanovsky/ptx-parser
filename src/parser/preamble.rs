use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while1},
    Parser,
    //multi::{many0, many1},
    sequence::{preceded, delimited},
    character::complete::{char, space0, space1, multispace1}, combinator::opt};

use super::{
    is_special, Preamble, Comment,
    comment::{parse_line_comment, comments_or_whitespace}};

#[derive(Debug, PartialEq)]
pub(super) struct Version<'a> {
    major: &'a str,
    minor: &'a str,
}

#[derive(Debug, PartialEq)]
pub(super) struct Target<'a> {
    target: &'a str,
}

#[derive(Debug, PartialEq)]
pub(super) struct AddressSize<'a> {
    size: &'a str,
}

fn parse_preamble(input: &str) -> IResult<&str, Preamble> {
    opt(comments_or_whitespace)
    .and(parse_version)
    .map(|a| a.1)
    .and(comments_or_whitespace)
    .map(|a| a.0)
    .and(parse_target)
    .and(comments_or_whitespace)
    .map(|a| a.0)
    .and(parse_address_size)
    .map(
        |((version, target), address_size)|
        Preamble { version, target, address_size }
    )
    .parse(input)
}


fn parse_version(input: &str) -> IResult<&str, Version> {
    preceded(
        tag(".version").and(space1),
        take_while1(char::is_numeric)
        .and(preceded(
            char('.'),
            take_while1(char::is_numeric)
        ))
        .map(|(major, minor)| Version { major, minor })
    )(input)
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    preceded(
        tag(".target").and(space1),
        take_while1(|c: char| !c.is_whitespace() && !is_special(c))
        .map(|target| Target { target })
    )(input)
}

fn parse_address_size(input: &str) -> IResult<&str, AddressSize> {
    preceded(
        tag(".address_size").and(space1),
        take_while1(|c: char| !c.is_whitespace() && !is_special(c))
        .map(|size| AddressSize { size })
    )(input)
}

#[cfg(test)]
mod test_parse_version {
    use super::*;

    #[test]
    fn no_whitespace() {
        assert_eq!(
            parse_version(".version 1.0"),
            Ok(("", Version { major: "1", minor: "0" }))
        );
    }

    #[test]
    fn leading_whitespace() {
        assert!(
            parse_version("  .version 1.0").is_err()
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse_version(".version 1.0  "),
            Ok(("  ", Version { major: "1", minor: "0" }))
        );
    }

    #[test]
    fn immediate_comment() {
        assert_eq!(
            parse_version(".version 1.0// This is a comment"),
            Ok(("// This is a comment", Version { major: "1", minor: "0" }))
        );
    }
}

#[cfg(test)]
mod test_parse_target {
    use super::{parse_target, Target};
    
    #[test]
    fn no_whitespace() {
        assert_eq!(
            parse_target(".target sm_30"),
            Ok(("", Target { target: "sm_30" }))
        );
    }

    #[test]
    fn leading_whitespace() {
        assert!(
            parse_target("  .target sm_30").is_err()
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse_target(".target sm_30  "),
            Ok(("  ", Target { target: "sm_30" }))
        );
    }

    #[test]
    fn immediate_comment() {
        assert_eq!(
            parse_target(".target sm_30// This is a comment"),
            Ok(("// This is a comment", Target { target: "sm_30" }))
        );
    }
}

#[cfg(test)]
mod test_parse_address_size {
    use super::{parse_address_size, AddressSize};
    
    #[test]
    fn no_whitespace() {
        assert_eq!(
            parse_address_size(".address_size 64"),
            Ok(("", AddressSize { size: "64" }))
        );
    }

    #[test]
    fn leading_whitespace() {
        assert!(
            parse_address_size("  .address_size 64").is_err()
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse_address_size(".address_size 64  "),
            Ok(("  ", AddressSize { size: "64" }))
        );
    }

    #[test]
    fn immediate_comment() {
        assert_eq!(
            parse_address_size(".address_size 64// This is a comment"),
            Ok(("// This is a comment", AddressSize { size: "64" }))
        );
    }
}

#[cfg(test)]
mod test_parse_preamble {
    use super::{parse_preamble, Preamble, Version, Target, AddressSize};

    #[test]
    fn no_whitespace() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64"),
            Ok(("", (Preamble {
                version: Version { major: "1", minor: "0" },
                target: Target { target: "sm_30" },
                address_size: AddressSize { size: "64" }
            })))
        );
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(
            parse_preamble("  .version 1.0\n.target sm_30\n.address_size 64"),
            Ok(("", (Preamble {
                version: Version { major: "1", minor: "0" },
                target: Target { target: "sm_30" },
                address_size: AddressSize { size: "64" }
            })))
        );
    }
    
    #[test]
    fn leading_newline() {
        assert_eq!(
            parse_preamble(" \n .version 1.0\n.target sm_30\n.address_size 64"),
            Ok(("", (Preamble {
                version: Version { major: "1", minor: "0" },
                target: Target { target: "sm_30" },
                address_size: AddressSize { size: "64" }
            })))
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64  "),
            Ok(("  ", (Preamble {
                version: Version { major: "1", minor: "0" },
                target: Target { target: "sm_30" },
                address_size: AddressSize { size: "64" }
            })))
        );
    }

    #[test]
    fn immediate_comment() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64// This is a comment"),
            Ok(("// This is a comment", (Preamble {
                version: Version { major: "1", minor: "0" },
                target: Target { target: "sm_30" },
                address_size: AddressSize { size: "64" }
            })))
        );
    }

    #[test]
    fn trailing_comment() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64\n// This is a comment"),
            Ok(("\n// This is a comment", (Preamble {
                version: Version { major: "1", minor: "0" },
                target: Target { target: "sm_30" },
                address_size: AddressSize { size: "64" }
            })))
        );
    }
}