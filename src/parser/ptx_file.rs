use nom::{branch::alt, Parser, character::complete::multispace0, sequence::preceded, multi::many0_count};

use super::{PtxFile, Function, Global, function::parse_function, global::parse_global, preamble::parse_preamble, comment::comment_or_whitespace};

pub(crate) enum FunctionOrGlobal<'a> {
    Function(Function<'a>),
    Global(Global<'a>),
}

impl<'a> Iterator for PtxFile<'a> {
    type Item = FunctionOrGlobal<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match preceded(
            many0_count(comment_or_whitespace),
            alt((
                parse_function.map(FunctionOrGlobal::Function),
                parse_global.map(FunctionOrGlobal::Global),
            )))
        .parse(self.body) {
            Ok((body, value)) => {
                self.body = body;
                Some(value)
            },
            Err(e) => todo!("Error: {e:?}")
            //Err(_) => None,
        }
    }
}

impl<'a> TryFrom<&'a str> for PtxFile<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (body, preamble) = preceded(
            many0_count(comment_or_whitespace), 
            parse_preamble
        )(value)?;
        Ok(PtxFile { preamble, body })
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test_iterator {
    use super::{PtxFile, FunctionOrGlobal};
    use crate::parser::{Function, Global};
    use crate::ptx_files::{_EXAMPLE_FILE, kernel, a, b, c, d};

    #[test]
    fn parse_example() {
        let ptx: PtxFile = _EXAMPLE_FILE.try_into().unwrap();
        println!("Preamble: {preamble:?}", preamble = ptx.preamble);
        for foo in ptx {
            match foo {
                FunctionOrGlobal::Function(function) => {
                    println!("Function: {function:?}");
                },
                FunctionOrGlobal::Global(global) => {
                    println!("Global: {global:?}");
                },
            }
        }
    }

    #[test]
    fn parse_kernel() {
        let ptx: PtxFile = kernel::_PTX.try_into().unwrap();
        println!("Preamble: {preamble:?}", preamble = ptx.preamble);
        for foo in ptx {
            match foo {
                FunctionOrGlobal::Function(function) => {
                    println!("Function: {function:?}");
                },
                FunctionOrGlobal::Global(global) => {
                    println!("Global: {global:?}");
                },
            }
        }
    }
}