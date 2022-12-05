use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use std::str::FromStr;

pub struct Interval(u8, u8);

impl Interval {
    fn new(start: u8, end: u8) -> Self {
        Self(start, end)
    }

    fn inside(&self, other: &Interval) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_range(input: &str) -> IResult<&str, Interval> {
    map(separated_pair(number, tag("-"), number), |(start, end)| {
        Interval::new(start, end)
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, (Interval, Interval)> {
    separated_pair(parse_range, tag(","), parse_range)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(Interval, Interval)>> {
    separated_list1(tag("\n"), parse_line)(input)
}

pub fn parse_input(input: &str) -> Vec<(Interval, Interval)> {
    match all_consuming(terminated(parse_lines, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[(Interval, Interval)]) -> usize {
    input
        .iter()
        .filter(|(x, y)| x.inside(y) || y.inside(x))
        .count()
}

pub fn part2(input: &[(Interval, Interval)]) -> usize {
    input.iter().filter(|(x, y)| x.overlaps(y)).count()
}
