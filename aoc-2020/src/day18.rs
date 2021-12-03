use crate::parsers::number;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space0},
    combinator::all_consuming,
    error::Error,
    multi::{fold_many0, separated_list1},
    sequence::{delimited, pair},
    Finish, IResult,
};

fn apply(op: char, a: i64, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => panic!("Unsupported operator"),
    }
}

#[aoc(day18, part1)]
fn part1(input: &str) -> Result<i64, Error<String>> {
    fn parens(input: &str) -> IResult<&str, i64> {
        delimited(tag("("), expr, tag(")"))(input)
    }

    fn factor(input: &str) -> IResult<&str, i64> {
        delimited(space0, alt((number, parens)), space0)(input)
    }

    fn expr(input: &str) -> IResult<&str, i64> {
        let (input, init) = factor(input)?;

        fold_many0(
            pair(one_of("*/+-"), factor),
            move || init,
            |acc, (op, val)| apply(op, acc, val),
        )(input)
    }

    match all_consuming(separated_list1(tag("\n"), expr))(input).finish() {
        Ok((_, output)) => Ok(output.iter().sum()),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day18, part2)]
fn part2(input: &str) -> Result<i64, Error<String>> {
    fn parens(input: &str) -> IResult<&str, i64> {
        delimited(tag("("), expr, tag(")"))(input)
    }

    fn factor(input: &str) -> IResult<&str, i64> {
        delimited(space0, alt((number, parens)), space0)(input)
    }

    fn expr2(input: &str) -> IResult<&str, i64> {
        let (input, init) = factor(input)?;

        fold_many0(
            pair(one_of("+-"), factor),
            move || init,
            |acc, (op, val)| apply(op, acc, val),
        )(input)
    }

    fn expr(input: &str) -> IResult<&str, i64> {
        let (input, init) = expr2(input)?;

        fold_many0(
            pair(one_of("*/"), expr2),
            move || init,
            |acc, (op, val)| apply(op, acc, val),
        )(input)
    }

    match all_consuming(separated_list1(tag("\n"), expr))(input).finish() {
        Ok((_, output)) => Ok(output.iter().sum()),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}
