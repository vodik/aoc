use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    error::Error,
    multi::separated_list1,
    Finish, IResult,
};
use std::str::FromStr;

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_group(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag("\n"), number)(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(tag("\n\n"), parse_group)(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    match parse_groups(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|group| group.iter().sum()).max().unwrap()
}

pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut ordered = input.iter().map(|group| group.iter().sum()).collect::<Vec<u32>>();
    ordered.sort_unstable();
    ordered.iter().rev().take(3).sum()
}
