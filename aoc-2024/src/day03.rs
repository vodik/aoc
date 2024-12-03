use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::{all_consuming, map, map_res, recognize},
    error::Error,
    multi::{fold_many0, many0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub enum Op {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_digit(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), |digit_str: &str| {
        digit_str.parse::<u32>()
    })(input)
}

fn parse_mul(input: &str) -> IResult<&str, Op> {
    map(
        preceded(
            tag("mul"),
            delimited(
                char('('),
                separated_pair(parse_digit, char(','), parse_digit),
                char(')'),
            ),
        ),
        |(d1, d2)| Op::Mul(d1, d2),
    )(input)
}

pub fn parse_opts(input: &str) -> IResult<&str, Vec<Op>> {
    fold_many0(
        alt((
            map(parse_mul, |parsed| Some(parsed)),
            map(tag("do()"), |_| Some(Op::Do)),
            map(tag("don't()"), |_| Some(Op::Dont)),
            map(anychar, |_| None),
        )),
        Vec::new,
        |mut acc, item| {
            if let Some(op) = item {
                acc.push(op);
            }
            acc
        },
    )(input)
}

pub fn parse_input(input: &str) -> Vec<Op> {
    match all_consuming(parse_opts)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[Op]) -> u32 {
    input
        .iter()
        .map(|op| match op {
            Op::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

pub fn part2(input: &[Op]) -> u32 {
    let mut enabled = true;
    input
        .iter()
        .map(|op| match &op {
            Op::Mul(a, b) => (enabled as u32) * a * b,
            Op::Do => {
                enabled = true;
                0
            }
            Op::Dont => {
                enabled = false;
                0
            }
        })
        .sum()
}
