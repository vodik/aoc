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

pub fn slices<T>(data: &[T]) -> impl Iterator<Item = &[T]> + '_ {
    data.iter().enumerate().map(move |(idx, _)| &data[idx..])
}

#[aoc(day9, part2)]
fn part2(data: &[u64]) -> Option<u64> {
    let weakness = part1(data)?;

    slices(data)
        .find_map(|slice| {
            let mut sum = 0;

            for (idx, x) in slice.iter().enumerate() {
                sum += x;

                match sum.cmp(&weakness) {
                    Ordering::Greater => break,
                    Ordering::Equal => return Some(&slice[..idx]),
                    _ => {}
                }
            }

            None
        })
        .and_then(|slice| {
            let min = slice.iter().min()?;
            let max = slice.iter().max()?;
            Some(min + max)
        })
}
