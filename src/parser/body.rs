use nom::{IResult, branch::alt, Parser, bytes::complete::take_while1};

pub(crate) struct Body<'a> {
    body: Option<&'a str>,
}

impl<'a> Iterator for Body<'a> {
    type Item = BodyLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Register<'a> {
    raw_string: &'a str,
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    todo!()
}

fn parse_unknown(input: &str) -> IResult<&str, &str> {
    take_while1(|_| true)(input)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Operation<'a> {
    operation: OperationKind,
    raw_string: &'a str,
}

fn parse_body_line(input: &str) -> IResult<&str, BodyLine> {
    let foo = alt((
        parse_unknown
        .map(BodyLine::Unknown),
    ))
    (input)?;
    todo!()
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