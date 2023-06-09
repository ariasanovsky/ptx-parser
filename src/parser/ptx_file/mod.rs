use nom::{branch::alt, combinator::opt, sequence::preceded, IResult, Parser};

use super::{
    function::{Function, parse::parse_function},
    global::{Global, parse::parse_global},
    PtxParser,
    comment::parse::many1_comments_or_whitespace,
};

mod try_from;

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
                let body = self.body?.trim();
                if body.is_empty() {
                    self.body = None;
                    return None
                } else {
                    self.body = Some(body);
                    err
                }
            }
        })
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test_iterator {
    use super::PtxParser;
    use crate::ptx_files::{kernel, _EXAMPLE_FILE};

    #[test]
    fn parse_example() {
        let ptx: PtxParser = _EXAMPLE_FILE.try_into().unwrap();
        dbg!(&ptx.preamble);
        for function_or_global in ptx {
            let _ = dbg!(function_or_global);
        }
    }

    #[test]
    fn parse_kernel() {
        let ptx: PtxParser = kernel::_PTX.try_into().unwrap();
        dbg!("Preamble: {:?}", &ptx.preamble);
        for function_or_global in ptx {
            let _ = dbg!(function_or_global);
        }
    }
}
