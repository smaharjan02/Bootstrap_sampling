

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, tag_no_case},
    character::complete::{char, space0, digit1},
    combinator::{opt, recognize},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    function: String,
    table: String,
    alias: String,
    where_cond_column: String,
    where_cond_comparator: String,
    where_cond_value: String,
    and_cond_column: String,
    and_cond_comparator: String,
    and_cond_value: String,

}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_' || c.is_uppercase())(input)
}


fn parse_integer(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn parse_float(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(digit1),
        char('.'),
        opt(tuple((digit1, opt(digit1)))),
    )))(input)
}

fn parse_number(input: &str) -> IResult<&str, &str> {
    alt((
        parse_float,
        parse_integer,

    ))(input)
}



fn parse_select(input:&str) -> IResult<&str, Option<&str>> {
    let (input , _) = space0(input)?;
    let (input, _) = tag_no_case("select")(input)?;
    let (input, _) = space0(input)?;

    let (input, function) = opt(terminated(tag_no_case("Count(*)"), space0))(input)?;
    let (input, _) = space0(input)?;

    Ok((input, function))

}

fn parse_from(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = tag_no_case("from")(input)?;
    let (input, _) = space0(input)?;

    let (input, table) = identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, alias) = identifier(input)?; 
    let (input, _) = space0(input)?;

    Ok((input, (table,alias)))
}

fn parse_where(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, _) = tag_no_case("where")(input)?;
    let (input, _) = space0(input)?;

    let (input, where_cond_column) = identifier(input)?;
    let (input, _) = space0(input)?;

    let (input, where_cond_comparator) = alt((tag("="), tag("<"), tag(">")))(input)?;
    let (input, _) = space0(input)?;

    let (input, where_cond_value) = parse_number(input)?;
    let (input, _) = space0(input)?;

    Ok((input, (where_cond_column, where_cond_comparator, where_cond_value)))
}

fn parse_and(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, _) = space0(input)?;
    let (input, _) = tag_no_case("and")(input)?;
    let (input, _) = space0(input)?;

    let (input, and_cond_column) = identifier(input)?;
    let (input, _) = space0(input)?;

    let (input, and_cond_comparator) = alt((tag("="), tag("<"), tag(">")))(input)?;
    let (input, _) = space0(input)?;

    let (input, and_cond_value) = parse_number(input)?;
    let (input, _) = space0(input)?;

    Ok((input, (and_cond_column, and_cond_comparator, and_cond_value)))
}

pub fn parse_select_statement (input: &str) -> IResult<&str, SelectStatement> {
    let (input , function) = parse_select(input).unwrap();

    let (input, (table, alias),)   = parse_from(input).unwrap();

    

    let (input, (where_cond_column, where_cond_comparator, where_cond_value)) = parse_where(input).unwrap();

    let (input, (and_cond_column, and_cond_comparator, and_cond_value)) = parse_and(input).unwrap();

    let statement = SelectStatement {
        function: function.unwrap_or("").to_string(),
        table: table.to_string(),
        alias: alias.to_string(),
        where_cond_column: where_cond_column.to_string(),
        where_cond_comparator: where_cond_comparator.to_string(),
        where_cond_value: where_cond_value.to_string(),
        and_cond_column: and_cond_column.to_string(),
        and_cond_comparator: and_cond_comparator.to_string(),
        and_cond_value: and_cond_value.to_string(),
    };

    Ok((input, statement))

}




