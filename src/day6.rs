use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    Finish, IResult,
};
use std::collections::HashSet;

struct AnswerSheet(Vec<HashSet<char>>);

impl AnswerSheet {
    fn union(&self) -> HashSet<char> {
        self.0.iter().flat_map(|set| set.iter()).cloned().collect()
    }

    fn intersection(&self) -> HashSet<char> {
        if let Some(first) = self.0.get(0) {
            self.0[1..].iter().fold(first.clone(), |acc, set| {
                acc.intersection(&set).cloned().collect()
            })
        } else {
            HashSet::new()
        }
    }
}

fn parse_answer(input: &str) -> IResult<&str, AnswerSheet> {
    map(
        separated_list1(tag("\n"), map(alpha1, |s: &str| s.chars().collect())),
        AnswerSheet,
    )(input)
}

fn parse_answers(input: &str) -> IResult<&str, Vec<AnswerSheet>> {
    separated_list1(tag("\n\n"), parse_answer)(input)
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Result<Vec<AnswerSheet>, Error<String>> {
    match all_consuming(parse_answers)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day6, part1)]
fn part1(data: &[AnswerSheet]) -> usize {
    data.iter().map(|sheet| sheet.union().len()).sum()
}

#[aoc(day6, part2)]
fn part2(data: &[AnswerSheet]) -> usize {
    data.iter().map(|sheet| sheet.intersection().len()).sum()
}
