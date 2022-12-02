use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    error::Error,
    multi::separated_list1,
    sequence::terminated,
    Finish, IResult,
};
use std::str::FromStr;

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_group(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag("\n"), number)(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(tag("\n\n"), parse_group)(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    match all_consuming(terminated(parse_groups, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn iter_packs(packs: &[Vec<u32>]) -> impl Iterator<Item = u32> + '_ {
    packs.iter().map(|pack| pack.iter().sum())
}

pub fn part1(input: &[Vec<u32>]) -> u32 {
    iter_packs(input).max().unwrap()
}

pub fn part2(input: &[Vec<u32>]) -> u32 {
    let top = iter_packs(input).fold([0; 3], |mut top, pack| {
        if pack > top[0] {
            top[0] = pack;
            top.sort_unstable();
        }
        top
    });

    top.iter().sum()
}
