use nom::{combinator::opt, sequence::preceded};

use crate::parser::{comment::parse::many1_comments_or_whitespace, preamble::parse::parse_preamble, PtxFile};

use super::PtxParser;

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

// impl<'a> TryFrom<&'a str> for PtxFile<'a> {
//     type Error = nom::Err<nom::error::Error<&'a str>>;

//     fn try_from(value: &'a str) -> Result<Self, Self::Error> {
//         let ptx: PtxParser = value.try_into()?;
//         ptx.try_into()
//     }
// }

// impl<'a> TryFrom<PtxParser<'a>> for PtxFile<'a> {
//     type Error = nom::Err<nom::error::Error<&'a str>>;

//     fn try_from(value: PtxParser<'a>) -> Result<Self, Self::Error> {
//         let mut functions = Vec::new();
//         let mut globals = Vec::new();
//         for function_or_global in value {
//             match function_or_global {
//                 Ok((_, super::FunctionOrGlobal::Function(function))) => functions.push(function),
//                 Ok((_, super::FunctionOrGlobal::Global(global))) => globals.push(global),
//                 Err(err) => return Err(err),
//             }
//         }

//         let preamble = value.preamble;

//         Ok(PtxFile { preamble, functions, globals })
//     }
// }