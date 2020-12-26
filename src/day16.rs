use crate::parsers::{number, range};
use nom::{
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{all_consuming, recognize},
    error::Error,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

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
    let (input, name) = recognize(many1(satisfy(|c| c.is_alphabetic() || c == ' ')))(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, ranges) = separated_pair(range, tag(" or "), range)(input)?;

    let ((start1, end1), (start2, end2)) = ranges;
    Ok((
        input,
        Rule {
            name: name.to_string(),
            range1: start1..=end1,
            range2: start2..=end2,
        },
    ))
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

#[aoc(day16, part1)]
fn part1((rules, _, neighbours): &Document) -> u32 {
    let valid = rules
        .iter()
        .flat_map(|rule| rule.iter_range())
        .collect::<HashSet<_>>();

    neighbours
        .iter()
        .flat_map(|ticket| ticket.iter().filter(|field| !valid.contains(field)))
        .sum()
}

#[aoc(day16, part2)]
fn part2((rules, ticket, neighbours): &Document) -> u64 {
    let valid = rules
        .iter()
        .flat_map(|rule| rule.iter_range())
        .collect::<HashSet<_>>();

    let valid_neighbours = neighbours
        .iter()
        .filter(|ticket| ticket.iter().all(|field| valid.contains(field)))
        .collect::<Vec<_>>();

    let expected_len = valid_neighbours[0].len();
    let mut fields = vec![HashSet::new(); expected_len];
    let mut possibilities: HashMap<_, HashSet<_>> = HashMap::new();

    for ticket in valid_neighbours {
        assert_eq!(ticket.len(), expected_len);

        for (idx, field) in ticket.iter().enumerate() {
            fields[idx].insert(*field);
        }
    }

    for rule in rules {
        for (idx, fieldset) in fields.iter().enumerate() {
            if fieldset.iter().all(|value| rule.contains(value)) {
                possibilities
                    .entry(rule.name.clone())
                    .or_insert_with(Default::default)
                    .insert(idx);
            }
        }
    }

    loop {
        let unique = possibilities
            .values()
            .filter(|&set| set.len() == 1)
            .flat_map(|set| set.clone())
            .collect::<HashSet<_>>();

        if unique.len() == possibilities.len() {
            break;
        }

        for set in possibilities.values_mut() {
            if set.len() == 1 {
                continue;
            }

            set.retain(|field| !unique.contains(field));
        }
    }

    possibilities
        .iter()
        .filter_map(|(key, set)| {
            if key.starts_with("departure") {
                set.iter().next().cloned()
            } else {
                None
            }
        })
        .map(|idx| ticket[idx] as u64)
        .product()
}
