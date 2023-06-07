use nom::{bytes::complete::take_while1, character::complete::char, sequence::delimited, IResult};

pub(crate) mod comment;
pub(crate) mod function;
pub(crate) mod global;
pub(crate) mod preamble;
pub(crate) mod ptx_file;

#[derive(Debug)]
pub struct PtxParser<'a> {
    preamble: Preamble<'a>,
    body: Option<&'a str>,
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct PtxFile<'a> {
    preamble: Preamble<'a>,
    functions: Vec<Function<'a>>,
    globals: Vec<Global<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Comment<'a> {
    Line(&'a str),
    Block(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Preamble<'a> {
    version: preamble::Version<'a>,
    target: preamble::Target<'a>,
    address_size: preamble::AddressSize<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    signature: function::FunctionSignature<'a>,
    body: Option<function::body::FunctionBody<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Global<'a> {
    raw_string: &'a str,
}

fn is_special(c: char) -> bool {
    ['.', '/', '(', ')', '[', ']', '{', '}', ',', ';', ':', '%']
    .contains(&c)
}

pub(crate) fn parse_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && !is_special(c))(input)
}

pub(crate) fn parse_parenthesized_naive(input: &str) -> IResult<&str, &str> {
    delimited(
        char('('),
        take_while1(|c: char| c != ')'),
        char(')')
    )(input)
}

pub(crate) fn _parse_braced_naive(input: &str) -> IResult<&str, &str> {
    delimited(
        char('{'),
        take_while1(|c: char| c != '}'),
        char('}')
    )(input)
}

pub(crate) fn parse_braced_balanced(input: &str) -> IResult<&str, &str> {
    let mut chars = input.chars().enumerate();
    let (mut depth, mut end) = match chars.next() {
        Some((_, '{')) => (1, None),
        _ => return Err(nom::Err::Error(
            nom::error::Error::new(input, nom::error::ErrorKind::Char)
        ))
    };

    for (i, c) in chars {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(i);
                    break;
                }
            }
            _ => (),
        }
    }
    if let Some(end) = end {
        Ok((&input[end + 1..], &input[1..end]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )))
    }
}

#[cfg(test)]
mod test_parse_parenthesized {

    use super::parse_parenthesized_naive;

    #[test]
    fn no_newline() {
        let input = "(hello)";
        let expected = Ok(("", "hello"));
        assert_eq!(parse_parenthesized_naive(input), expected)
    }

    #[test]
    fn newline() {
        let input = "(hello\n)";
        let expected = Ok(("", "hello\n"));
        assert_eq!(parse_parenthesized_naive(input), expected)
    }

    #[test]
    fn one_left_parenthesis() {
        let input = "(hello";
        assert!(parse_parenthesized_naive(input).is_err())
    }

    #[test]
    fn two_left_one_right() {
        let input = "((hello)";
        assert_eq!(
            parse_parenthesized_naive(input),
            Ok(("", "(hello")),
        )
    }
}

#[cfg(test)]
mod test_parse_braced {

    use super::_parse_braced_naive;

    #[test]
    fn no_newline() {
        let input = "{hello}";
        let expected = Ok(("", "hello"));
        assert_eq!(_parse_braced_naive(input), expected)
    }

    #[test]
    fn newline() {
        let input = "{hello\n}";
        let expected = Ok(("", "hello\n"));
        assert_eq!(_parse_braced_naive(input), expected)
    }

    #[test]
    fn one_left_brace() {
        let input = "{hello";
        assert!(_parse_braced_naive(input).is_err())
    }

    #[test]
    fn two_left_one_right() {
        let input = "{{hello}";
        assert_eq!(
            _parse_braced_naive(input),
            Ok(("", "{hello")),
        )
    }

    #[test]
    fn mock_function_body() {
        let input = "{.reg .b32 %r<3>}";
        let expected = Ok(("", ".reg .b32 %r<3>"));
        assert_eq!(_parse_braced_naive(input), expected)
    }
}

#[cfg(test)]
mod test_parse_braced_balanced {

    use super::parse_braced_balanced;

    #[test]
    fn one_pair() {
        let input = "{hello}";
        let expected = Ok(("", "hello"));
        assert_eq!(parse_braced_balanced(input), expected)
    }

    #[test]
    fn two_pairs() {
        let input = "{hello}{world}";
        let expected = Ok(("{world}", "hello"));
        assert_eq!(parse_braced_balanced(input), expected)
    }

    #[test]
    fn nested_pair() {
        let input = "{hello{world}}";
        let expected = Ok(("", "hello{world}"));
        assert_eq!(parse_braced_balanced(input), expected)
    }

    #[test]
    fn imbalanced() {
        let input = "{hello{world}";
        assert!(parse_braced_balanced(input).is_err())
    }
}
