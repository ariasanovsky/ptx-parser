#[derive(Debug, PartialEq)]
pub struct Global<'a> {
    raw_string: &'a str,
}

pub(crate) mod parse;

#[cfg(test)]
mod test_parse_global {
    use crate::parser::global::{Global, parse::parse_global};

    #[test]
    fn trivial_exaample() {
        let input = ".global hello;";
        let expected = Ok(("", Global { raw_string: "hello" }));
        assert_eq!(parse_global(input), expected)
    }
}
