use itertools::unfold;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};
use std::str::FromStr;

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(number, tag("\n"), number)(input)
}

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Result<(u64, u64), Error<String>> {
    match all_consuming(parse_numbers)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn transform(subject: u64) -> impl Iterator<Item = u64> {
    let mut value = 1;
    unfold((), move |_| {
        value = (value % 20201227) * (subject % 20201227) % 20201227;
        Some(value)
    })
}

fn crack(target: u64, iterations: usize) -> Option<usize> {
    transform(7)
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
fn part1((card_key, door_key): &(u64, u64)) -> Option<u64> {
    let loop_size = crack(*card_key, 100_000_000).unwrap();
    transform(*door_key).skip(loop_size - 1).next()
}
