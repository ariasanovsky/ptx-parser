use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    Parser,
    sequence::{preceded, delimited},
    character::complete::{char, space0, space1, multispace0},
    combinator::{opt, value},branch::alt
};

use super::{parse_name, parse_parenthesized_naive};

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
        opt(parse_parenthesized_naive)
    )(input)?;
    
    let (input, name) = parse_name(input)?;

    let (input, parameters) = preceded(
        multispace0,
        opt(parse_parenthesized_naive)
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
    fn func_no_return_trivial_parameters() {
        let input =
".func _ZN4core9panicking(hi)";
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
                    parameters: Some("hi"),
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
                    parameters: Some("
	.param .b64 _ZN4core9panicking_param_0,
	.param .b64 _ZN4core9panicking_param_1,
	.param .b64 _ZN4core9panicking_param_2
"
                    ),
                }
            ))
        )
    }
}

fn multiline_delimited(input: &str) -> IResult<&str, &str> {
    delimited(
        char('('), 
        take_while1(|c: char| c != ')'),
        char(')')
    )(input)
}

#[test]
fn test_multiline_delimited() {
    let input = "(hello\nworld!)";
    let delimited = multiline_delimited(input);
    assert_eq!(delimited, Ok(("", "hello\nworld!")))
}