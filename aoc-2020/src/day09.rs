use crate::iter::tails;
use std::{cmp::Ordering, num::ParseIntError};

const PREAMBLE_LEN: usize = 25;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day9, part1)]
fn part1(data: &[u64]) -> Option<u64> {
    data.windows(PREAMBLE_LEN + 1).find_map(|window| {
        let target = window[PREAMBLE_LEN];

        if tails(&window[..PREAMBLE_LEN])
            .flat_map(|(x, xs)| xs.iter().map(move |y| x + y))
            .any(|sum| sum == target)
        {
            None
        } else {
            Some(target)
        }
    })
}

#[aoc(day9, part2)]
fn part2(data: &[u64]) -> Option<u64> {
    let weakness = part1(data)?;

    let mut start = 0;
    let mut end = 1;
    let mut sum = data[0] + data[1];

    let slice = loop {
        match sum.cmp(&weakness) {
            Ordering::Less => {
                end += 1;
                sum += data[end];
            }
            Ordering::Greater => {
                sum -= data[start];
                start += 1;
            }
            Ordering::Equal => break &data[start..=end],
        }
    };

    let min = slice.iter().min()?;
    let max = slice.iter().max()?;
    Some(min + max)
}
