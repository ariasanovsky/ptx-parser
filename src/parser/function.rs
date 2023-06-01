use nom::{
    IResult,
    bytes::complete::tag,
    Parser,
    sequence::preceded,
    character::complete::{space0, space1, multispace0},
    combinator::{opt, value},
    branch::alt
};

use super::{parse_name, parse_parenthesized_naive};

#[derive(Debug, PartialEq)]
pub(super) struct FunctionSignature<'a> {
    visible: bool,
    entry: bool,
    return_value: Option<ReturnValue<'a>>,
    name: &'a str,
    parameters: Option<Parameters<'a>>,
}

#[derive(Debug, PartialEq)]
pub(super) struct ReturnValue<'a> {
    raw_string: &'a str,
}

#[derive(Debug, PartialEq)]
pub(super) struct Parameters<'a> {
    raw_string: &'a str,
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

#[cfg(test)]
mod test_parse_function_signature {

    use super::{parse_function_signature, FunctionSignature, ReturnValue, Parameters};

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
                    parameters: Some(Parameters { raw_string: "hi" }),
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
                    parameters: Some(Parameters { raw_string: "
	.param .b64 _ZN4core9panicking_param_0,
	.param .b64 _ZN4core9panicking_param_1,
	.param .b64 _ZN4core9panicking_param_2
"}),
                }
            ))
        )
    }

    #[test]
    fn func_return_and_parameters() {
        let input =
".func  (.param .b64 func_retval0) _foo(
	.param .b64 _foo_param_0,
	.param .b64 _foo_param_1
)";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: Some(ReturnValue { raw_string: ".param .b64 func_retval0" }),
                    name: "_foo",
                    parameters: Some(Parameters { raw_string: "
	.param .b64 _foo_param_0,
	.param .b64 _foo_param_1
"})
                }
            ))
        )
    }
}
