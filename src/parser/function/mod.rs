pub(super) mod body;
pub(crate) mod parse;

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    signature: FunctionSignature<'a>,
    body: Option<body::FunctionBody<'a>>,
}

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

#[cfg(test)]
mod test_parse_function_signature {

    use crate::parser::function::parse::parse_function_signature;

    use super::{FunctionSignature, ReturnValue, Parameters};

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

#[cfg(test)]
mod test_parse_function_body {

    use crate::parser::function::{
        parse::parse_function_body, body::FunctionBody,
    };

    #[test]
    fn empty() {
        let input = ";";
        let body = parse_function_body(input);
        assert!(
            body.is_err()
        )
    }

    #[test]
    fn non_empty() {
        let input = "{.reg .b32 %r<3>}";
        let body = parse_function_body(input);
        assert_eq!(
            body,
            Ok((
                "",
                FunctionBody { body: Some(".reg .b32 %r<3>") }
            ))
        )
    }
}

#[cfg(test)]
mod test_parse_function {
    use crate::parser::function::{parse::parse_function, Function, FunctionSignature, body::FunctionBody};


    #[test]
    fn no_return_no_parameters_no_body() {
        let input = ".func _Z6kernelPiS_i;";
        let function = parse_function(input);
        assert_eq!(
            function,
            Ok((
                "",
                Function {
                    signature: FunctionSignature {
                        visible: false,
                        entry: false,
                        return_value: None,
                        name: "_Z6kernelPiS_i",
                        parameters: None,
                    },
                    body: None,
                }
            ))
        )
    }

    #[test]
    fn no_return_no_parameters_with_body() {
        let input = ".func _Z6kernelPiS_i { \n foo \n bar }";
        let function = parse_function(input);
        assert_eq!(
            function,
            Ok((
                "",
                Function {
                    signature: FunctionSignature {
                        visible: false,
                        entry: false,
                        return_value: None,
                        name: "_Z6kernelPiS_i",
                        parameters: None,
                    },
                    body: Some(FunctionBody { body: Some(" \n foo \n bar ") }),
                }
            ))
        )
    }
}