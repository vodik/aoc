use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, satisfy},
    combinator::{all_consuming, map_res, recognize},
    IResult,
};
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub args: (u32, u32),
    pub target: u8,
}

impl Rule {
    fn as_range(&self) -> RangeInclusive<u32> {
        self.args.0..=self.args.1
    }
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, arg0) = number(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, arg1) = number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, alpha) = satisfy(|c| c.is_alphabetic())(input)?;

    Ok((
        input,
        Rule {
            args: (arg0, arg1),
            target: alpha as u8,
        },
    ))
}

fn parse_entry(input: &str) -> IResult<&str, (Rule, String)> {
    let (input, rule) = parse_rule(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, line) = alphanumeric1(input)?;

    Ok((input, (rule, String::from(line))))
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<(Rule, String)> {
    input
        .lines()
        .map(|line| {
            let (_, result) = all_consuming(parse_entry)(line).expect("Unable to parse password");
            result
        })
        .collect()
}

fn byte_freq(input: &[u8]) -> HashMap<u8, u32> {
    let mut counter = HashMap::new();
    for b in input {
        let count = counter.entry(*b).or_insert(0);
        *count += 1;
    }
    counter
}

#[aoc(day2, part1)]
fn part1(data: &[(Rule, String)]) -> usize {
    data.iter()
        .filter(|(rule, password)| {
            let counts = byte_freq(password.as_bytes());

            match counts.get(&rule.target) {
                Some(count) => rule.as_range().contains(count),
                None => false,
            }
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(data: &[(Rule, String)]) -> usize {
    data.iter()
        .filter(|(rule, password)| {
            let bytes = password.as_bytes();
            let first = bytes[rule.args.0 as usize - 1];
            let second = bytes[rule.args.1 as usize - 1];

            first != second && (first == rule.target || second == rule.target)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_parser() {
        assert_eq!(
            parse_rule("1-3 a"),
            Ok((
                "",
                Rule {
                    args: (1, 3),
                    target: 'a' as u8
                }
            ))
        );
    }

    #[test]
    fn entry_parser() {
        assert_eq!(
            parse_entry("4-9 b: cdefg"),
            Ok((
                "",
                (
                    Rule {
                        args: (4, 9),
                        target: 'b' as u8
                    },
                    "cdefg".into()
                )
            ))
        );
    }
}
