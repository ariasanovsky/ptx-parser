use nom::{
    bytes::complete::{tag, take_until},
    character::complete::space1,
    combinator::{map_res, map},
    sequence::{delimited, pair, preceded},
    IResult, error::context,
};

#[derive(Debug, PartialEq)]
struct NameValue<'a> {
    name: &'a str,
    value: &'a str,
}

fn parse_name_value(input: &str) -> IResult<&str, NameValue> {
    
    let (input, (name, value)) = 
    delimited(
        tag("."), 
        pair(
            map(
                take_until(" "),
                |s| s
            ),
            preceded(
                space1,
                context(
                    "value",
                    map(
                    take_until("\n"), 
                    |s| s
                    )
                )
            )
        ),
        tag("\n")
    )(input)?;

    Ok((input, NameValue { name, value }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name_value() {
        let input = ".name value\n";
        let expected_output = NameValue {
            name: "name",
            value: "value",
        };
        assert_eq!(parse_name_value(input), Ok(("", expected_output)));
    }
}