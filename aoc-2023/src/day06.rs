use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Races {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl Races {
    pub fn new(times: Vec<u64>, distances: Vec<u64>) -> Self {
        Self { times, distances }
    }

    pub fn races(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        self.times
            .iter()
            .copied()
            .zip(self.distances.iter().copied())
    }

    pub fn kerned_race(&self) -> (u64, u64) {
        (
            combine_numbers(&self.times),
            combine_numbers(&self.distances),
        )
    }
}

fn combine_numbers(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(String::new(), |mut buffer, &number| {
            buffer.push_str(&number.to_string());
            buffer
        })
        .parse()
        .unwrap()
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(space1, separated_list1(space1, number))(input)
}

fn parse_races(input: &str) -> IResult<&str, Races> {
    map(
        terminated(
            separated_pair(
                preceded(tag("Time:"), parse_numbers),
                tag("\n"),
                preceded(tag("Distance:"), parse_numbers),
            ),
            tag("\n"),
        ),
        |(times, distances)| Races::new(times, distances),
    )(input)
}

pub fn parse_input(input: &str) -> Races {
    match all_consuming(parse_races)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn simulate(time: u64, distance: u64) -> usize {
    (0..time + 1)
        .map(|held| {
            let speed = held;
            speed * (time - held)
        })
        .filter(|&moved| moved > distance)
        .count()
}

pub fn part1(input: &Races) -> usize {
    input
        .races()
        .map(|(time, distance)| simulate(time, distance))
        .product()
}

pub fn part2(input: &Races) -> usize {
    let (time, distance) = input.kerned_race();
    simulate(time, distance)
}
