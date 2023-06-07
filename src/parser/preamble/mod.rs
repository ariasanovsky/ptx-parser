pub(crate) mod parse;

#[derive(Debug, PartialEq)]
pub struct Preamble<'a> {
    version: Version<'a>,
    target: Target<'a>,
    address_size: AddressSize<'a>,
}

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

#[cfg(test)]
mod test_parse_version {
    use crate::parser::preamble::{Version, parse::parse_version};

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
    use crate::parser::preamble::{Target, parse::parse_target};

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
    use crate::parser::preamble::{AddressSize, parse::parse_address_size};

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
    use crate::parser::preamble::{parse::parse_preamble, Preamble, Version, Target, AddressSize};

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
