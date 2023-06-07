use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, space1},
    combinator::opt,
    sequence::{preceded, Tuple},
    IResult, Parser,
};

use super::{comment::many1_comments_or_whitespace, parse_name, Preamble};

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

pub(super) fn parse_preamble(input: &str) -> IResult<&str, Preamble> {
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

fn parse_version(input: &str) -> IResult<&str, Version> {
    (
        preceded(tag(".version").and(space1), take_while1(char::is_numeric)),
        preceded(char('.'), take_while1(char::is_numeric)),
    )
    .parse(input)
    .map(|(input, (major, minor))| (input, Version { major, minor }))
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    preceded(
        tag(".target").and(space1),
        parse_name.map(|target| Target { target }),
    )(input)
}

fn parse_address_size(input: &str) -> IResult<&str, AddressSize> {
    preceded(
        tag(".address_size").and(space1),
        parse_name.map(|size| AddressSize { size }),
    )(input)
}

#[cfg(test)]
mod test_parse_version {
    use super::*;

    #[test]
    fn no_whitespace() {
        assert_eq!(
            parse_version(".version 1.0"),
            Ok((
                "",
                Version {
                    major: "1",
                    minor: "0"
                }
            ))
        );
    }

    #[test]
    fn leading_whitespace() {
        assert!(parse_version("  .version 1.0").is_err());
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse_version(".version 1.0  "),
            Ok((
                "  ",
                Version {
                    major: "1",
                    minor: "0"
                }
            ))
        );
    }

    #[test]
    fn immediate_comment() {
        assert_eq!(
            parse_version(".version 1.0// This is a comment"),
            Ok((
                "// This is a comment",
                Version {
                    major: "1",
                    minor: "0"
                }
            ))
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
        assert!(parse_target("  .target sm_30").is_err());
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
        assert!(parse_address_size("  .address_size 64").is_err());
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
    use super::{parse_preamble, AddressSize, Preamble, Target, Version};

    #[test]
    fn no_whitespace() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64"),
            Ok((
                "",
                (Preamble {
                    version: Version {
                        major: "1",
                        minor: "0"
                    },
                    target: Target { target: "sm_30" },
                    address_size: AddressSize { size: "64" }
                })
            ))
        );
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(
            parse_preamble("  .version 1.0\n.target sm_30\n.address_size 64"),
            Ok((
                "",
                (Preamble {
                    version: Version {
                        major: "1",
                        minor: "0"
                    },
                    target: Target { target: "sm_30" },
                    address_size: AddressSize { size: "64" }
                })
            ))
        );
    }

    #[test]
    fn leading_newline() {
        assert_eq!(
            parse_preamble(" \n .version 1.0\n.target sm_30\n.address_size 64"),
            Ok((
                "",
                (Preamble {
                    version: Version {
                        major: "1",
                        minor: "0"
                    },
                    target: Target { target: "sm_30" },
                    address_size: AddressSize { size: "64" }
                })
            ))
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64  "),
            Ok((
                "  ",
                (Preamble {
                    version: Version {
                        major: "1",
                        minor: "0"
                    },
                    target: Target { target: "sm_30" },
                    address_size: AddressSize { size: "64" }
                })
            ))
        );
    }

    #[test]
    fn immediate_comment() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64// This is a comment"),
            Ok((
                "// This is a comment",
                (Preamble {
                    version: Version {
                        major: "1",
                        minor: "0"
                    },
                    target: Target { target: "sm_30" },
                    address_size: AddressSize { size: "64" }
                })
            ))
        );
    }

    #[test]
    fn trailing_comment() {
        assert_eq!(
            parse_preamble(".version 1.0\n.target sm_30\n.address_size 64\n// This is a comment"),
            Ok((
                "\n// This is a comment",
                (Preamble {
                    version: Version {
                        major: "1",
                        minor: "0"
                    },
                    target: Target { target: "sm_30" },
                    address_size: AddressSize { size: "64" }
                })
            ))
        );
    }
}
