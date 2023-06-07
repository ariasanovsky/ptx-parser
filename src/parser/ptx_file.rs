use nom::{branch::alt, combinator::opt, sequence::preceded, IResult, Parser};

use super::{
    comment::many1_comments_or_whitespace, function::parse_function, global::parse_global,
    preamble::parse_preamble, Function, Global, PtxParser,
};

#[derive(Debug)]
pub enum FunctionOrGlobal<'a> {
    Function(Function<'a>),
    Global(Global<'a>),
}

impl<'a> Iterator for PtxParser<'a> {
    type Item = IResult<&'a str, FunctionOrGlobal<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let body = self.body?;
        Some(match preceded(
                opt(many1_comments_or_whitespace),
                alt((
                parse_function
                .map(FunctionOrGlobal::Function),
                parse_global
                .map(FunctionOrGlobal::Global),
        )))(body) {
            Ok((body, value)) => {
                self.body = Some(body);
                Ok((body, value))
            }
            err => {
                self.body = None;
                err
            }
        })
    }
}

impl<'a> TryFrom<&'a str> for PtxParser<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (body, preamble) = preceded(
            opt(many1_comments_or_whitespace), 
            parse_preamble
        )(value)?;
        Ok(PtxParser { preamble, body: Some(body) })
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test_iterator {
    use super::{FunctionOrGlobal, PtxParser};
    use crate::parser::Function;
    use crate::ptx_files::{kernel, _EXAMPLE_FILE};

    impl<'a> FunctionOrGlobal<'a> {
        pub(crate) fn function(self) -> Option<Function<'a>> {
            match self {
                FunctionOrGlobal::Function(function) => Some(function),
                _ => None,
            }
        }
    }

    #[test]
    fn parse_example() {
        let ptx: PtxParser = _EXAMPLE_FILE.try_into().unwrap();
        dbg!("Preamble: {:?}", &ptx.preamble);
        for _function_or_global in ptx {
            dbg!("{_function_or_global:?}\n");
        }
    }

    #[test]
    fn parse_kernel() {
        let ptx: PtxParser = kernel::_PTX.try_into().unwrap();
        dbg!("Preamble: {:?}", &ptx.preamble);
        for _function_or_global in ptx {
            dbg!("{_function_or_global:?}\n");
        }
    }
}
