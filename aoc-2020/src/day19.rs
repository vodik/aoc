use crate::parsers::number;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, satisfy},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::{collections::HashMap, iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
enum Rule {
    OneOf(Vec<Vec<u64>>),
    Match(char),
}

type Input = (HashMap<u64, Rule>, Vec<String>);

fn parse_chain(input: &str) -> IResult<&str, Rule> {
    map(
        separated_list1(tag(" | "), separated_list1(tag(" "), number)),
        Rule::OneOf,
    )(input)
}

fn parse_match(input: &str) -> IResult<&str, Rule> {
    delimited(
        tag("\""),
        map(satisfy(|c| c.is_alphabetic()), Rule::Match),
        tag("\""),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (u64, Rule)> {
    separated_pair(number, tag(": "), alt((parse_chain, parse_match)))(input)
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<u64, Rule>> {
    map(separated_list1(tag("\n"), parse_rule), |rules| {
        rules.into_iter().collect()
    })(input)
}

fn parse_input2(input: &str) -> IResult<&str, Input> {
    separated_pair(
        parse_rules,
        tag("\n\n"),
        separated_list1(tag("\n"), map(alpha1, |s: &str| s.to_string())),
    )(input)
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Result<Input, Error<String>> {
    match all_consuming(parse_input2)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn match_rule(rules: &HashMap<u64, Rule>, rule: u64, chars: &mut Peekable<Chars>) -> bool {
    match rules.get(&rule) {
        Some(Rule::Match(target)) => chars.next().map_or(false, |c| *target == c),
        Some(Rule::OneOf(alts)) => alts.iter().any(|alt| {
            let mut recursing = false;
            let mut tmp_chars = chars.clone();

            let matched = alt
                .iter()
                .all(|sub_rule| match (tmp_chars.peek(), recursing) {
                    (None, recursing) => recursing,
                    _ => {
                        recursing = *sub_rule == rule;
                        match_rule(rules, *sub_rule, &mut tmp_chars)
                    }
                });

            if matched {
                *chars = tmp_chars;
            }
            matched
        }),
        None => panic!(),
    }
}

fn match_rules(rules: &HashMap<u64, Rule>, input: &str) -> bool {
    let mut chars = input.chars().peekable();
    match_rule(rules, 0, &mut chars) && chars.peek().is_none()
}

#[aoc(day19, part1)]
fn part1((rules, lines): &Input) -> usize {
    lines.iter().filter(|line| match_rules(rules, line)).count()
}

#[aoc(day19, part2)]
fn part2((rules, lines): &Input) -> usize {
    let mut rules = rules.clone();
    rules.insert(8, Rule::OneOf(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::OneOf(vec![vec![42, 31], vec![42, 11, 31]]));

    lines
        .iter()
        .filter(|line| match_rules(&rules, line))
        .count()
}
