use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
use std::collections::HashSet;

type Node = (String, Vec<(u64, String)>);

fn description(input: &str) -> IResult<&str, String> {
    let (input, desc) = recognize(separated_pair(alpha1, tag(" "), alpha1))(input)?;
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = opt(tag("s"))(input)?;

    Ok((input, desc.to_string()))
}

fn target(input: &str) -> IResult<&str, (u64, String)> {
    let (input, count) = map_res(digit1, str::parse::<u64>)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, desc) = description(input)?;

    Ok((input, (count, desc)))
}

fn rule(input: &str) -> IResult<&str, (String, Vec<(u64, String)>)> {
    let (input, desc) = description(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, targets) = alt((
        map(tag("no other bags"), |_| Vec::new()),
        separated_list1(tag(", "), target),
    ))(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((input, (desc, targets)))
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
