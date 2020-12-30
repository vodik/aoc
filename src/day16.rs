use crate::parsers::{number, range};
use nom::{
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{all_consuming, map, recognize},
    error::Error,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

type Document = (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>);

#[derive(Debug)]
struct Rule {
    name: String,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

impl Rule {
    fn contains(&self, target: &u32) -> bool {
        self.range1.contains(target) || self.range2.contains(target)
    }

    fn iter_range(&self) -> impl Iterator<Item = u32> {
        self.range1.clone().chain(self.range2.clone())
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(
            recognize(many1(satisfy(|c| c.is_alphabetic() || c == ' '))),
            tag(": "),
            separated_pair(range, tag(" or "), range),
        ),
        |(name, ((start1, end1), (start2, end2)))| Rule {
            name: name.to_string(),
            range1: start1..=end1,
            range2: start2..=end2,
        },
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag("\n"), parse_rule)(input)
}

fn parse_ticket(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), number)(input)
}

fn parse_data(input: &str) -> IResult<&str, Document> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = tag("\n\nyour ticket:\n")(input)?;
    let (input, ticket) = parse_ticket(input)?;
    let (input, _) = tag("\n\nnearby tickets:\n")(input)?;
    let (input, neighbours) = separated_list1(tag("\n"), parse_ticket)(input)?;

    Ok((input, (rules, ticket, neighbours)))
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Result<Document, Error<String>> {
    match all_consuming(parse_data)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn valid_fields(rules: &[Rule]) -> HashSet<u32> {
    rules.iter().flat_map(|rule| rule.iter_range()).collect()
}

#[aoc(day16, part1)]
fn part1((rules, _, neighbours): &Document) -> u32 {
    let valid = valid_fields(rules);
    neighbours
        .iter()
        .flat_map(|ticket| ticket.iter().filter(|field| !valid.contains(field)))
        .sum()
}

#[aoc(day16, part2)]
fn part2((rules, ticket, neighbours): &Document) -> u64 {
    let valid = valid_fields(rules);
    let valid_neighbours = neighbours
        .iter()
        .filter(|ticket| ticket.iter().all(|field| valid.contains(field)))
        .collect::<Vec<_>>();

    let expected_len = valid_neighbours[0].len();
    let mut transpose = vec![HashSet::new(); expected_len];

    for ticket in valid_neighbours {
        for (idx, field) in ticket.iter().enumerate() {
            transpose[idx].insert(*field);
        }
    }

    let mut possibilities: HashMap<_, HashSet<_>> = HashMap::new();
    let mut matches = Vec::with_capacity(6);

    for rule in rules {
        for (idx, values) in transpose.iter().enumerate() {
            if values.iter().all(|value| rule.contains(value)) {
                possibilities
                    .entry(rule.name.clone())
                    .or_insert_with(Default::default)
                    .insert(idx);
            }
        }
    }

    loop {
        let unique = possibilities
            .drain_filter(|_, set| set.len() == 1)
            .flat_map(|(key, unique_set)| {
                unique_set
                    .into_iter()
                    .next()
                    .map(|position| (key, position))
            })
            .collect::<Vec<_>>();

        if unique.is_empty() {
            break;
        }

        for (key, value) in unique {
            for set in possibilities.values_mut() {
                set.remove(&value);
            }

            if key.starts_with("departure") {
                matches.push(value);
            }
        }
    }

    matches.into_iter().map(|idx| ticket[idx] as u64).product()
}
