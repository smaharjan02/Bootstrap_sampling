#![allow(dead_code)]
// Used libraries
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, tag_no_case},
    character::complete::{char, space0, digit1},
    combinator::{opt, recognize},
    sequence::{terminated, tuple},
    IResult,
};

//represents a parsed SQL statement
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

impl SelectStatement {
    pub fn function(&self) -> &str {
        &self.function
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }

    pub fn where_cond_column(&self) -> &str {
        &self.where_cond_column
    }

    pub fn where_cond_comparator(&self) -> &str {
        &self.where_cond_comparator
    }

    pub fn where_cond_value(&self) -> &str {
        &self.where_cond_value
    }

    pub fn and_cond_column(&self) -> &str {
        &self.and_cond_column
    }

    pub fn and_cond_comparator(&self) -> &str {
        &self.and_cond_comparator
    }

    pub fn and_cond_value(&self) -> &str {
        &self.and_cond_value
    }


}

//parses a SQL identifier
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphanumeric() || c == '_' || c.is_uppercase())(input)
}

//parsing an integer
fn parse_integer(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

//parsing a float (.5, 10.55)
fn parse_float(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        opt(digit1),
        char('.'),
        opt(tuple((digit1, opt(digit1)))),
    )))(input)
}
//combining the two functions above to parse a number
fn parse_number(input: &str) -> IResult<&str, &str> {
    alt((
        parse_float,
        parse_integer,

    ))(input)
}

//parsing the select and retrieving the count function
fn parse_select(input:&str) -> IResult<&str, Option<&str>> {
    let (input , _) = space0(input)?;
    let (input, _) = tag_no_case("select")(input)?;
    let (input, _) = space0(input)?;

    let (input, function) = opt(terminated(tag_no_case("Count(*)"), space0))(input)?;
    let (input, _) = space0(input)?;

    Ok((input, function))

}

//parsing from and retrieving the table name and alias
fn parse_from(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = tag_no_case("from")(input)?;
    let (input, _) = space0(input)?;

    let (input, table) = identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, alias) = identifier(input)?; 
    let (input, _) = space0(input)?;

    Ok((input, (table,alias)))
}

//parsing where and retrieving the column name, comparator and value
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

//parsing and and retrieving the column name, comparator and value
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

//combining all the functions above to parse a select statement
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




