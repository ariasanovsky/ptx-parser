pub(crate) mod token;
pub(crate) mod preamble;
pub(crate) mod comment;
pub(crate) mod function;

fn is_special(c: char) -> bool {
    ['.', '/', '(', ')', '[', ']', '{', '}', ',', ';', ':', '%'].contains(&c)
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Period,
    ForwardSlash,
    String(&'a str),
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Percent,
}


#[derive(Debug, PartialEq)]
pub(crate) enum Comment<'a> {
    Line(&'a str),
    Block(&'a str),
}

#[derive(Debug, PartialEq)]
struct Preamble<'a> {
    version: preamble::Version<'a>,
    target: preamble::Target<'a>,
    address_size: preamble::AddressSize<'a>,
}

#[derive(Debug, PartialEq)]
enum Function<'a> {
    Declaration( function::FunctionSignature<'a> ),
    Definition( function::FunctionSignature<'a>, function::FunctionBody<'a> ),
}
