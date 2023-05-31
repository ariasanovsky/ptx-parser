use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_while1},
    Parser,
    //multi::{many0, many1},
    sequence::{preceded, delimited},
    character::complete::{char, space0, space1, multispace1}, combinator::opt, branch::alt};

use crate::parser::is_special;

#[derive(Debug, PartialEq)]
pub(super) struct FunctionSignature<'a> {
    visible: bool,
    entry: bool,
    return_value: Option<&'a str>,
    name: &'a str,
    parameters: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub(super) struct FunctionBody<'a> {
    registers: &'a str,
    instructions: &'a str,
}

fn parse_function_signature(input: &str) -> IResult<&str, FunctionSignature> {
    let (input, (visible, entry)) = alt((
        tag(".visible")
        .and(space1)
        .and(tag(".entry"))
        .map(|_| (true, true)),
        tag(".func")
        .map(|_| (false, false))
    ))
    (input)?;
    
    let (input, return_value) = preceded(
        space1,
        opt(delimited(
            char('('),
            take_while1(|c: char| c != ')'),
            char(')').and(space0)
        ))
    )(input)?;
    
    let (input, name) = 
    take_while1(|c: char| !c.is_whitespace() && !is_special(c))
    (input)?;

    let (input, parameters) = preceded(
        space0,
        opt(delimited(
            char('('),
            take_while1(|c: char| c != ')'),
            char(')')
        ))
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

#[cfg(test)]
mod test_parse_function_signature {

    use super::{parse_function_signature, FunctionSignature};

    #[test]
    fn visible_entry_name() {
        let input = ".visible .entry _Z6kernelPiS_i";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: true,
                    entry: true,
                    return_value: None,
                    name: "_Z6kernelPiS_i",
                    parameters: None,
                }
            ))
        )
    }

    #[test]
    fn func_no_return_no_parameters() {
        let input = ".func _Z6kernelPiS_i";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: None,
                    name: "_Z6kernelPiS_i",
                    parameters: None,
                }
            ))
        )
    }

    #[test]
    fn func_no_return_some_parameters() {
        let input =
".func _ZN4core9panicking
(
	.param .b64 _ZN4core9panicking_param_0,
	.param .b64 _ZN4core9panicking_param_1,
	.param .b64 _ZN4core9panicking_param_2
)";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: None,
                    name: "_ZN4core9panicking",
                    parameters: Some(
                        "_ZN4core9panicking_param_0,\n\t_param .b64 _ZN4core9panicking_param_1,\n\t_param .b64 _ZN4core9panicking_param_2"
                    ),
                }
            ))
        )
    }
}