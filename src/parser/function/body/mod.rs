use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1, take_while1},
    character::complete::{char, multispace1, space0, space1},
    combinator::opt,
    sequence::{delimited, preceded, terminated, Tuple},
    IResult, Parser,
};

use crate::parser::{
    is_special,
    parse_braced_balanced, comment::{parse::{many1_comments_or_whitespace, parse_line_comment}, Comment}
};

#[derive(Debug, PartialEq)]
pub(super) struct FunctionBody<'a> {
    pub(super) body: Option<&'a str>,
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
    operation: &'a str,
    arguments: &'a str,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Goto<'a> {
    predicate: Option<Predicate<'a>>,
    label: &'a str,
}

fn parse_unknown_line(input: &str) -> IResult<&str, &str> {
    take_while1(|_| true)(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (operation, arguments)) = (
        take_while1(|c: char|
            !c.is_whitespace() && (c == '.' || !is_special(c))
        ),
        preceded(
            space1,
            take_while1(|_| true)
        ),
    )
    .parse(input)?;
    Ok((
        input,
        Operation {
            operation,
            arguments,
        },
    ))
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
            space1.and(tag("bra")).and(space1).and(char('$')),
        ),
        tag("bra.uni").and(space1).and(char('$'))
        .map(|_| None),
    ))
    (input)?;
    Ok((input, Goto { predicate, label }))
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionCall<'a> {
    setup: &'a str,
    function: &'a str,
    arguments: &'a str,
    comment: Comment<'a>,
}

fn parse_function_call(input: &str) -> IResult<&str, FunctionCall> {
    let (input, (body, comment)) = (
        parse_braced_balanced,
        preceded(space0, parse_line_comment)
    )
    .parse(input)?;

    (
        take_until1("call.uni"),
        delimited(
            tag("call.uni").and(multispace1),
            take_while1(|c: char| c != ','),
            char(','),
        ),
    )
        .parse(body)
        .map(|(arguments, (setup, function))| {
            (
                input,
                FunctionCall {
                    setup,
                    function,
                    arguments,
                    comment,
                },
            )
        })
}

fn parse_body_line(input: &str) -> IResult<&str, BodyLine> {
    let body_line = alt((
        delimited(
            char('$'),
            take_while1(|c: char| !c.is_whitespace() && c != ':'),
            char(':'),
        )
        .map(BodyLine::Label),
        parse_function_call
        .map(BodyLine::FunctionCall),
        terminated(
            alt((
                take_while1(|c: char| c != ';'),
            )),
            char(';')
        )
        .map(BodyLine::Unknown)
    ))
    (input)?;
    Ok(match body_line {
        (input, BodyLine::Unknown(raw_string)) => {
            let (_, body_line) = alt((
                tag("ret").map(|_| BodyLine::Return),
                parse_goto.map(BodyLine::Goto),
                parse_register.map(BodyLine::Register),
                parse_operation.map(BodyLine::Operation),
                parse_unknown_line.map(BodyLine::Unknown),
            ))
            .parse(raw_string)?;
            (input, body_line)
        }
        label_or_braced => label_or_braced,
    })
}

#[derive(Debug, PartialEq)]
pub(crate) enum BodyLine<'a> {
    Register(Register<'a>),
    Operation(Operation<'a>),
    Label(&'a str),
    Goto(Goto<'a>),
    Return,
    FunctionCall(FunctionCall<'a>),
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
        parser::{PtxParser, ptx_file::FunctionOrGlobal},
        ptx_files::{a, kernel, _EXAMPLE_FILE},
    };

    use super::{BodyLine, Operation};

    fn show_body_lines(input: &str) {
        let ptx: PtxParser = input.try_into().unwrap();
        ptx
        .into_iter()
            .filter_map(|line| line.ok())
        .filter_map(|(_, function)| {
            if let FunctionOrGlobal::Function(function) = function {
                Some(function)
            } else {
                None
            }
        })
        .for_each(|function| {
            dbg!("Function: {function:?}");
            if let Some(body) = function.body {
                for line in body {
                    if let Ok(line) = line {
                        dbg!("Body line: {:?}", line.1);
                    }
                }
            }
        });
    }

    fn show_unknown_body_lines(input: &str) {
        let ptx: PtxParser = input.try_into().unwrap();
        ptx
        .into_iter()
            .filter_map(|line| line.ok())
        .filter_map(|(_, function)| {
            if let FunctionOrGlobal::Function(function) = function {
                Some(function)
            } else {
                None
            }
        })
        .for_each(|function| {
            if let Some(body) = function.body {
                body.filter_map(Result::ok)
                    .map(|(_, line)| line)
                    .for_each(|line| {
                        if let BodyLine::Unknown(raw_string) = line {
                            dbg!("Unknown line: {:?}", raw_string);
                        }
                    })
            }
        });
    }

    impl<'a> BodyLine<'a> {
        pub(crate) fn operation(self) -> Option<Operation<'a>> {
            match self {
                BodyLine::Operation(operation) => Some(operation),
                _ => None,
            }
        }
    }

    fn show_operations(input: &str) {
        let ptx: PtxParser = input.try_into().unwrap();
        ptx
        .into_iter()
        .filter_map(|line| line.ok())
        .filter_map(|(_, function)| {
            if let FunctionOrGlobal::Function(function) = function {
                Some(function)
            } else {
                None
            }
        })
        .filter_map(|function| function.body)
        .for_each(|body| {
            body
            .filter_map(Result::ok)
            .map(|(_, line)| line)
            .filter_map(|line| line.operation())
            .for_each(|operation| {
                let Operation {
                    operation: _operation,
                    arguments: _arguments,
                } = operation;
                dbg!("Operation: {_operation} with arguments: {_arguments}");
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
        show_operations(a::_PTX)
    }
}
