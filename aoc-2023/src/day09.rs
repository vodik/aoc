use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res, opt, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Finish, IResult,
};
use std::{borrow::Cow, str::FromStr};

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(" "), number)(input)
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    terminated(separated_list1(tag("\n"), parse_sequence), tag("\n"))(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    match all_consuming(parse_sequences)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn deltas(sequence: &[i32]) -> impl Iterator<Item = i32> + '_ {
    sequence.windows(2).map(|window| window[1] - window[0])
}

fn predict_forwards(sequence: &[i32]) -> i32 {
    let mut edges = vec![*sequence.last().unwrap()];
    let mut iteration = Cow::from(sequence);
    loop {
        let next = deltas(&iteration).collect::<Vec<_>>();
        if next.iter().all(|&value| value == 0) {
            break;
        };
        edges.push(*next.last().unwrap());
        iteration = Cow::from(next);
    }
    edges.iter().sum()
}

fn predict_backwards(sequence: &[i32]) -> i32 {
    let mut edges = vec![*sequence.first().unwrap()];
    let mut iteration = Cow::from(sequence);
    loop {
        let next = deltas(&iteration).collect::<Vec<_>>();
        if next.iter().all(|&value| value == 0) {
            break;
        };
        edges.push(*next.first().unwrap());
        iteration = Cow::from(next);
    }
    edges.iter().rev().fold(0, |acc, value| value - acc)
}

pub fn part1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|sequence| predict_forwards(sequence))
        .sum()
}

pub fn part2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|sequence| predict_backwards(sequence))
        .sum()
}
