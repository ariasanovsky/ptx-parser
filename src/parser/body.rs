use nom::{
    IResult,
    branch::alt,
    bytes::complete::{take_while1, tag},
    character::complete::char, sequence::{delimited, terminated, preceded}, Parser, combinator::opt,
};

use crate::parser::comment::many1_comments_or_whitespace;

#[derive(Debug, PartialEq)]
pub(super) struct FunctionBody<'a> {
    pub(crate) body: Option<&'a str>,
}

impl<'a> Iterator for FunctionBody<'a> {
    type Item = IResult<&'a str, BodyLine<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let body = self.body?;
        Some(match preceded(
            opt(many1_comments_or_whitespace),
            parse_body_line,
        )(body) {
            Ok((body, value)) => {
                self.body = Some(body);
                Ok((body, value))
            },
            err => {
                self.body = None;
                err
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Register<'a> {
    raw_string: &'a str,
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    preceded(
        tag(".reg"),
        take_while1(|_| true)
        .map(|raw_string| Register { raw_string })
    )(input)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Operation<'a> {
    operation: OperationKind,
    raw_string: &'a str,
}

fn parse_unknown_line(input: &str) -> IResult<&str, &str> {
    take_while1(|_| true)
    (input)
}

fn parse_body_line(input: &str) -> IResult<&str, BodyLine> {
    let foo = alt((
        delimited(
            char('$'),
            take_while1(|c: char| !c.is_whitespace()),
            char(':')
        )
        .map(BodyLine::Label),
        terminated(
            alt((
                take_while1(|c: char| c != ';'),
            )),
            char(';')
        )
        .map(BodyLine::Unknown)
    ))
    (input)?;
    Ok(match foo {
        (input, BodyLine::Unknown(raw_string)) => {
            let (_, foo) = alt((
                parse_register
                .map(BodyLine::Register),
                parse_unknown_line
                .map(BodyLine::Unknown),
            ))
            .parse(raw_string)?;
            (input, foo)
        },
        label => label
    })
}

#[derive(Debug, PartialEq)]
pub(crate) enum BodyLine<'a> {
    Register(Register<'a>),
    Operation(Operation<'a>),
    Label(&'a str),
    Goto {
        label: &'a str,
        predicate: Option<Predicate<'a>>,
    },
    Return,
    Unknown(&'a str),
}

#[derive(Debug, PartialEq)]
pub(crate) enum OperationKind {
    LdParamU32,
    MovU32,
    MadLoS32,
    SetpLtS32,
    LdParamU64,
    CvtaToGlobalU64,
    MulWideS32,
    AddS64,
    LdGlobalF32,
    MulRnF32,
    StGlobalF32,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Predicate<'a> {
    True(&'a str),
    False(&'a str),
}

#[cfg(test)]
mod test_iterator {
    use crate::{parser::{PtxFile, ptx_file::FunctionOrGlobal}, ptx_files::_EXAMPLE_FILE};

    #[test]
    fn parse_body_example() {
        let mut ptx: PtxFile = _EXAMPLE_FILE.try_into().unwrap();
        let function = ptx.next().unwrap().unwrap().1;
        if let FunctionOrGlobal::Function(function) = function {
            println!("Function: {function:?}");
            for line in function.body.unwrap() {
                if let Ok(line) = line {
                    println!("Body line: {:?}", line.1);
                }
            }
        } else {
            panic!("Expected function")
        };
    }
}