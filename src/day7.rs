use crate::parsers::number;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, map, opt, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use std::collections::HashSet;

type Node = (String, Vec<(u64, String)>);

fn parse_bag(input: &str) -> IResult<&str, String> {
    map(
        terminated(
            recognize(separated_pair(alpha1, tag(" "), alpha1)),
            terminated(tag(" bag"), opt(tag("s"))),
        ),
        |s: &str| s.to_string(),
    )(input)
}

fn parse_bag_count(input: &str) -> IResult<&str, (u64, String)> {
    separated_pair(number, tag(" "), parse_bag)(input)
}

fn parse_bag_set(input: &str) -> IResult<&str, Vec<(u64, String)>> {
    alt((
        map(tag("no other bags"), |_| Default::default()),
        separated_list1(tag(", "), parse_bag_count),
    ))(input)
}

fn rule(input: &str) -> IResult<&str, (String, Vec<(u64, String)>)> {
    separated_pair(
        parse_bag,
        tag(" contain "),
        terminated(parse_bag_set, tag(".")),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(String, Vec<(u64, String)>)>> {
    separated_list1(tag("\n"), rule)(input)
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Result<Vec<Node>, Error<String>> {
    match all_consuming(parse_rules)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn find_holders<'a>(data: &'a [Node], desc: &'a str) -> impl Iterator<Item = &'a String> + 'a {
    data.iter().filter_map(move |(node, rules)| {
        if rules.iter().any(|(_, rule)| desc == rule) {
            Some(node)
        } else {
            None
        }
    })
}

#[aoc(day7, part1)]
fn part1(data: &[Node]) -> usize {
    let mut holders = find_holders(data, "shiny gold").collect::<HashSet<_>>();

    let mut level = holders.clone();
    loop {
        let new_level = level
            .into_iter()
            .flat_map(|t| find_holders(data, t))
            .collect::<HashSet<_>>();

        if new_level.is_empty() {
            break;
        }

        holders.extend(&new_level);
        level = new_level;
    }
    holders.len()
}

fn find_rules<'a>(data: &'a [Node], desc: &'a str) -> &'a [(u64, String)] {
    data.iter()
        .find_map(
            |(node, rules)| {
                if node == desc {
                    Some(rules)
                } else {
                    None
                }
            },
        )
        .unwrap()
}

fn follow_rules(data: &[Node], rules: &[(u64, String)]) -> u64 {
    rules
        .iter()
        .map(|(count, desc)| {
            let rules = find_rules(data, desc);
            let x = follow_rules(data, rules);
            count + count * x
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(data: &[Node]) -> u64 {
    let rules = find_rules(data, "shiny gold");
    follow_rules(data, rules)
}
