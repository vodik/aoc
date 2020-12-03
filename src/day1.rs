use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn heads<T>(data: &[T]) -> impl Iterator<Item = (&T, &[T])> + '_ {
    data.iter()
        .enumerate()
        .map(move |(idx, v)| (v, &data[idx + 1..]))
}

#[aoc(day1, part1)]
fn part1(data: &[u32]) -> Option<u32> {
    heads(data)
        .flat_map(move |(&x, xs)| xs.iter().map(move |&y| (x, y)))
        .find_map(|(x, y)| if x + y == 2020 { Some(x * y) } else { None })
}

#[aoc(day1, part1, hashed)]
fn part1_hashed(data: &[u32]) -> Option<u32> {
    let set = data.iter().cloned().collect::<HashSet<_>>();

    data.iter().find_map(|value| {
        2020u32.checked_sub(*value).and_then(|target| {
            if set.contains(&target) {
                Some(value * target)
            } else {
                None
            }
        })
    })
}

#[aoc(day1, part2)]
fn part2(data: &[u32]) -> Option<u32> {
    heads(data)
        .flat_map(move |(&x, xs)| {
            heads(xs).flat_map(move |(&y, ys)| ys.iter().map(move |&z| (x, y, z)))
        })
        .find_map(|(x, y, z)| {
            if x + y + z == 2020 {
                Some(x * y * z)
            } else {
                None
            }
        })
}

#[aoc(day1, part2, hashed)]
fn part2_hashed(data: &[u32]) -> Option<u32> {
    let set = data.iter().cloned().collect::<HashSet<_>>();

    heads(data)
        .flat_map(move |(&x, xs)| xs.iter().map(move |&y| (x, y)))
        .find_map(|(x, y)| {
            2020u32
                .checked_sub(x)
                .and_then(|target| target.checked_sub(y))
                .and_then(|target| {
                    if set.contains(&target) {
                        Some(x * y * target)
                    } else {
                        None
                    }
                })
        })
}
