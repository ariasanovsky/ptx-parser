use nom::{
    IResult,
    branch::alt,
    bytes::complete::{take_while1, tag},
    character::complete::{char, space1}, sequence::{delimited, terminated, preceded, Tuple}, Parser, combinator::{opt, value},
};

use crate::parser::comment::many1_comments_or_whitespace;

use super::is_special;

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
        tag(".reg").and(space1),
        take_while1(|_| true)
        .map(|raw_string| Register { raw_string })
    )(input)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Operation<'a> {
    operation: OperationKind<'a>,
    arguments: &'a str,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Goto<'a> {
    predicate: Option<Predicate<'a>>,
    label: &'a str,
}

fn parse_unknown_line(input: &str) -> IResult<&str, &str> {
    take_while1(|_| true)
    (input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (operation, arguments)) = (
        take_while1(|c: char|
            !c.is_whitespace() && (c == '.' || !is_special(c))
        ),
        preceded(
            space1,
            take_while1(|_| true)
        )
    )
    .parse(input)?;
    let operation = match operation {
        "ld.param.u32" => OperationKind::LdParamU32,
        "mov.u32" => OperationKind::MovU32,
        "mad.lo.s32" => OperationKind::MadLoS32,
        "setp.lt.s32" => OperationKind::SetpLtS32,
        "ld.param.u64" => OperationKind::LdParamU64,
        "cvta.to.global.u64" => OperationKind::CvtaToGlobalU64,
        "mul.wide.s32" => OperationKind::MulWideS32,
        "add.s64" => OperationKind::AddS64,
        "ld.global.f32" => OperationKind::LdGlobalF32,
        "mul.rn.f32" => OperationKind::MulRnF32,
        "st.global.f32" => OperationKind::StGlobalF32,
        "mov.u64" => OperationKind::MovU64,
        "cvta.global.u64" => OperationKind::CvtaGlobalU64,
        "setp.ge.s32" => OperationKind::SetpGeS32,
        "cvt.s64.s32" => OperationKind::CvtS64S32,
        "add.u64" => OperationKind::AddU64,
        "mul.lo.s32" => OperationKind::MulLoS32,
        operataion => OperationKind::Unknown(operataion),
    };
    Ok((input, Operation { operation, arguments }))
}

fn parse_goto(input: &str) -> IResult<&str, Goto> {
    let (label, predicate) = alt((
        delimited(
            char('@'),
            opt(char('!'))
            .and(take_while1(|c: char| !c.is_whitespace()))
            .map(|(negation, raw_string)| {
                Some(if negation.is_none() {
                    Predicate::True(raw_string)
                } else {
                    Predicate::False(raw_string)
                })
            }),
            space1.and(tag("bra")).and(space1).and(char('$'))
        ),
        tag("bra.uni").and(space1).and(char('$'))
        .map(|_| None),
    ))
    (input)?;
    Ok((input, Goto { predicate, label }))
}

fn parse_body_line(input: &str) -> IResult<&str, BodyLine> {
    let foo = alt((
        delimited(
            char('$'),
            take_while1(|c: char| !c.is_whitespace() && c != ':'),
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
                tag("ret")
                .map(|_| BodyLine::Return),
                preceded(
                    char('{')
                    .and(space1)
                    .and(tag("//"))
                    .and(space1)
                    .and(tag("callseq"))
                    .and(space1),
                    take_while1(|c: char| c != '\n')
                    .map(|raw_string| BodyLine::FunctionCallEntry(raw_string))
                ),
                preceded(
                    char('}')
                    .and(space1)
                    .and(tag("//"))
                    .and(space1)
                    .and(tag("callseq"))
                    .and(space1),
                    take_while1(|c: char| c != '\n')
                    .map(|raw_string| BodyLine::FunctionCallExit(raw_string))
                ),
                parse_goto
                .map(BodyLine::Goto),
                parse_register
                .map(BodyLine::Register),
                parse_operation
                .map(BodyLine::Operation),
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
    Goto(Goto<'a>),
    Return,
    FunctionCallEntry(&'a str),
    FunctionCallExit(&'a str),
    Unknown(&'a str),
}

impl<'a> BodyLine<'a> {
    pub(crate) fn operation(self) -> Option<Operation<'a>> {
        match self {
            BodyLine::Operation(operation) => Some(operation),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum OperationKind<'a> {
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
    MovU64,
    CvtaGlobalU64,
    SetpGeS32,
    CvtS64S32,
    AddU64,
    MulLoS32,
    Unknown(&'a str),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Predicate<'a> {
    True(&'a str),
    False(&'a str),
}

#[cfg(test)]
mod test_iterator {
    use crate::{
        parser::{
            PtxFile, ptx_file::FunctionOrGlobal
        },
        ptx_files::{_EXAMPLE_FILE, kernel, a
        }
    };

    use super::{BodyLine, OperationKind, Operation};

    fn show_body_lines(input: &str) {
        let ptx: PtxFile = input.try_into().unwrap();
        ptx
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|(_, function)| {
            function.function()
        })
        .for_each(|function| {
            println!("Function: {function:?}");
            if let Some(body) = function.body {
                for line in body {
                    if let Ok(line) = line {
                        println!("Body line: {:?}", line.1);
                    }
                }
            }
        })
        ;
    }

    fn show_unknown_body_lines(input: &str) {
        let ptx: PtxFile = input.try_into().unwrap();
        ptx
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|(_, function)| {
            function.function()
        })
        .for_each(|function| {
            if let Some(body) = function.body {
                body.filter_map(Result::ok)
                .map(|(_, line)| line)
                .for_each(|line| {
                    if let BodyLine::Unknown(raw_string) = line {
                        println!("Unknown line: {:?}", raw_string);
                    }
                })
            }
        })
        ;
    }

    fn show_unknown_operations(input: &str) {
        let ptx: PtxFile = input.try_into().unwrap();
        ptx
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|(_, function)| {
            function.function()
        })
        .filter_map(|function| function.body)
        .for_each(|body| {
            body.filter_map(Result::ok)
            .map(|(_, line)| line)
            .filter_map(|line| line.operation())
            .for_each(|operation| {
                let Operation { operation, arguments} = operation;
                if let OperationKind::Unknown(raw_string) = operation {
                    println!("Unknown: {raw_string} with arguments: {arguments}");
                }
            })
        })
    }

    #[test]
    fn parse_body_example() {
        show_body_lines(_EXAMPLE_FILE)
    }

    #[test]
    fn parse_body_kernel() {
        show_body_lines(kernel::_PTX)
    }

    #[test]
    fn parse_body_a() {
        show_body_lines(a::_PTX)
    }

    #[test]
    fn parse_unknowns_a() {
        show_unknown_body_lines(a::_PTX)
    }

    #[test]
    fn parse_unknowns_kernel() {
        show_unknown_body_lines(kernel::_PTX)
    }

    #[test]
    fn parse_unknown_operations_a() {
        show_unknown_operations(a::_PTX)
    }
}