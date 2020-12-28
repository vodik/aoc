use crate::parsers::number;
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
    zeros_mask: u64,
    ones_mask: u64,
}

impl DecoderV1 {
    fn new(input: &str) -> Self {
        let zeros_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c != '0' { 1 } else { 0 });

        let ones_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c == '1' { 1 } else { 0 });

        Self {
            zeros_mask,
            ones_mask,
        }
    }

    fn decode(&self, value: u64) -> u64 {
        value & self.zeros_mask | self.ones_mask
    }
}

struct DecoderV2 {
    zeros_mask: u64,
    ones_mask: u64,
    floating_masks: Vec<u64>,
}

fn floating_masks(input: &str) -> Vec<u64> {
    let ops = input
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(idx, c)| if c == 'X' { Some(idx) } else { None })
        .enumerate()
        .collect::<Vec<_>>();

    let (max_bits, _) = ops.last().unwrap();
    let max_value = 1 << (max_bits + 1);

    (0..max_value)
        .map(|base| {
            ops.iter().fold(0, |acc, (mask, offset)| {
                let bit = base & 1 << mask;
                let shift = offset - mask;
                acc | bit << shift
            })
        })
        .collect()
}

impl DecoderV2 {
    fn new(input: &str) -> Self {
        let zeros_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c != 'X' { 1 } else { 0 });

        let ones_mask = input
            .chars()
            .fold(0, |acc, c| acc << 1 | if c == '1' { 1 } else { 0 });

        Self {
            zeros_mask,
            ones_mask,
            floating_masks: floating_masks(input),
        }
    }

    fn decode(&self, value: u64) -> impl Iterator<Item = u64> + '_ {
        let base = value & self.zeros_mask | self.ones_mask;
        self.floating_masks
            .iter()
            .map(move |float_mask| base | float_mask)
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
                let decoder = decoder.as_ref().expect("No decoded has been set");
                memory.insert(*addr, decoder.decode(*value));
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
                let decoder = decoder.as_ref().expect("No decoded has been set");
                for addr in decoder.decode(*addr) {
                    memory.insert(addr, *value);
                }
            }
        }
    }

    memory.values().sum()
}
