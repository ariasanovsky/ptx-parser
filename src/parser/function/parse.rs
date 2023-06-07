use nom::{
    IResult,
    bytes::complete::tag,
    Parser,
    sequence::preceded,
    character::complete::{space0, space1, multispace0},
    combinator::{opt, value, map},
    branch::alt,
    character::complete::char,
};

use crate::parser::{comment::parse::many1_comments_or_whitespace, parse_braced_balanced, parse_parenthesized_naive, parse_name};

use super::{Function, body::FunctionBody, FunctionSignature, ReturnValue, Parameters};

pub(crate) fn parse_function(input: &str) -> IResult<&str, Function> {
    let (input, signature) = 
    parse_function_signature(input)?;
    let (input, body) = preceded(
        opt(many1_comments_or_whitespace),
        alt((
            map(
                char(';'),
                |_| None
            ),
            parse_function_body
            .map(Some)
        ))
    )(input)?;
    Ok((
        input,
        Function {
            signature,
            body,
        }
    ))
}

pub(crate) fn parse_function_body(input: &str) -> IResult<&str, FunctionBody> {
    parse_braced_balanced
        .map(|raw_string| FunctionBody { body: Some(raw_string) })
    .parse(input)
}

pub(super) fn parse_function_signature(input: &str) -> IResult<&str, FunctionSignature> {
    let (input, (visible, entry)) = alt((
        value(
            (true, true),
            tag(".visible")
            .and(space1)
            .and(tag(".entry"))
        ),
        value(
            (false, false),
            tag(".func")
        )
    ))
    (input)?;
    
    let (input, return_value) = preceded(
        space1,
        opt(
            parse_parenthesized_naive
            .map(|raw_string| ReturnValue { raw_string })
        )
    )(input)?;

    let (input, name) = preceded(
        space0,
        parse_name
    )(input)?;

    let (input, parameters) = preceded(
        multispace0,
        opt(
            parse_parenthesized_naive
            .map(|raw_string| Parameters { raw_string })
        )
    )(input)?;

    Ok((
        input,
        FunctionSignature {
            visible,
            entry,
            return_value,
            name,
            parameters,
        }
    ))
}
