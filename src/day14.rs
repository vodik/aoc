use crate::parsers::number;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    SetMask(String),
    Write(u64, u64),
}

fn is_mask(c: char) -> bool {
    c == 'X' || c == '0' || c == '1'
}

fn parse_mask(input: &str) -> IResult<&str, Op> {
    map(
        preceded(tag("mask = "), take_while_m_n(36, 36, is_mask)),
        |s: &str| Op::SetMask(s.to_string()),
    )(input)
}

fn parse_write(input: &str) -> IResult<&str, Op> {
    map(
        separated_pair(delimited(tag("mem["), number, tag("]")), tag(" = "), number),
        |(addr, value)| Op::Write(addr, value),
    )(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(tag("\n"), alt((parse_mask, parse_write)))(input)
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Result<Vec<Op>, Error<String>> {
    match all_consuming(parse_program)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

struct DecoderV1 {
    clear_mask: u64,
    set_mask: u64,
}

impl DecoderV1 {
    fn new(input: &str) -> Self {
        let clear_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c != '0' { 1 } else { 0 });

        let set_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c == '1' { 1 } else { 0 });

        Self {
            clear_mask,
            set_mask,
        }
    }
    fn decode(&self, value: u64) -> u64 {
        value & self.clear_mask | self.set_mask
    }
}

struct DecoderV2 {
    clear_mask: u64,
    set_masks: Vec<u64>,
}

impl DecoderV2 {
    fn new(input: &str) -> Self {
        let clear_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c != 'X' { 1 } else { 0 });

        let set_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c == '1' { 1 } else { 0 });

        let floats = input
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == 'X')
            .map(|(idx, _)| 1u64 << (input.len() - idx - 1))
            .collect::<Vec<_>>();

        let mut set_masks = vec![set_mask];
        for size in 1..=floats.len() {
            for combination in floats.iter().combinations(size) {
                set_masks.push(combination.iter().fold(set_mask, |mask, &&x| mask | x));
            }
        }

        Self {
            clear_mask,
            set_masks,
        }
    }

    fn decode(&self, value: u64) -> impl Iterator<Item = u64> + '_ {
        let base = value & self.clear_mask;
        self.set_masks.iter().map(move |mask| base | mask)
    }
}

#[aoc(day14, part1)]
fn part1(data: &[Op]) -> u64 {
    let mut memory = HashMap::new();
    let mut decoder = None;

    for op in data {
        match op {
            Op::SetMask(mask) => decoder = Some(DecoderV1::new(mask)),
            Op::Write(addr, value) => {
                memory.insert(*addr, decoder.as_ref().unwrap().decode(*value));
            }
        }
    }

    memory.values().sum()
}

#[aoc(day14, part2)]
fn part2(data: &[Op]) -> u64 {
    let mut memory = HashMap::new();
    let mut decoder = None;

    for op in data {
        match op {
            Op::SetMask(mask) => decoder = Some(DecoderV2::new(mask)),
            Op::Write(addr, value) => {
                for addr in decoder.as_ref().unwrap().decode(*addr) {
                    memory.insert(addr, *value);
                }
            }
        }
    }

    memory.values().sum()
}
