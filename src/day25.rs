use crate::parsers::number;
use nom::{
    bytes::complete::tag, combinator::all_consuming, error::Error, sequence::separated_pair,
    Finish, IResult,
};

const MAX_ITERATIONS: usize = 1_000_000_000;

const SHARED_BASE: u64 = 7;
const SHARED_MOD: u64 = 20201227;

fn parse_keys(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(number, tag("\n"), number)(input)
}

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Result<(u64, u64), Error<String>> {
    match all_consuming(parse_keys)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

struct Transform {
    value: u64,
    subject: u64,
}

impl Transform {
    fn new(subject: u64) -> Self {
        Self { value: 1, subject }
    }
}

impl Iterator for Transform {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value % SHARED_MOD) * (self.subject % SHARED_MOD) % SHARED_MOD;
        Some(self.value)
    }
}

fn crack(target: u64, iterations: usize) -> Option<usize> {
    Transform::new(SHARED_BASE)
        .take(iterations)
        .enumerate()
        .find_map(|(count, value)| {
            if value == target {
                Some(count + 1)
            } else {
                None
            }
        })
}

#[aoc(day25, part1)]
fn part1(&(card_key, door_key): &(u64, u64)) -> Option<u64> {
    let loop_size = crack(card_key, MAX_ITERATIONS).unwrap();
    Transform::new(door_key).nth(loop_size - 1)
}
