use crate::parsers::range;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, satisfy},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub args: (usize, usize),
    pub target: u8,
}

impl Rule {
    fn as_range(&self) -> RangeInclusive<usize> {
        self.args.0..=self.args.1
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(range, tag(" "), satisfy(|c| c.is_alphabetic())),
        |(args, alpha)| Rule {
            args,
            target: alpha as u8,
        },
    )(input)
}

fn parse_entry(input: &str) -> IResult<&str, (Rule, Vec<u8>)> {
    map(
        separated_pair(parse_rule, tag(": "), alphanumeric1),
        |(rule, line)| (rule, line.as_bytes().to_vec()),
    )(input)
}

fn parse_entries(input: &str) -> IResult<&str, Vec<(Rule, Vec<u8>)>> {
    separated_list1(tag("\n"), parse_entry)(input)
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Result<Vec<(Rule, Vec<u8>)>, Error<String>> {
    match all_consuming(parse_entries)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day2, part1)]
fn part1(data: &[(Rule, Vec<u8>)]) -> usize {
    data.iter()
        .filter(|(rule, password)| {
            let count = bytecount::count(password, rule.target);
            rule.as_range().contains(&count)
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(data: &[(Rule, Vec<u8>)]) -> usize {
    data.iter()
        .filter(|(rule, password)| {
            let first = password[rule.args.0 - 1];
            let second = password[rule.args.1 - 1];
            first != second && (first == rule.target || second == rule.target)
        })
        .count()
}
