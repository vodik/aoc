use std::{collections::HashMap, num::ParseIntError};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

fn play(data: &[u32], target: usize) -> u32 {
    let mut history: HashMap<u32, usize> = data
        .iter()
        .enumerate()
        .map(|(idx, value)| (*value, idx + 1))
        .collect();

    (data.len() + 1..target).fold(0, |current, turn| {
        history
            .insert(current, turn)
            .map_or(0, |previous| (turn - previous) as u32)
    })
}

#[aoc(day15, part1)]
fn part1(data: &[u32]) -> u32 {
    play(data, 2020)
}

#[aoc(day15, part2)]
fn part2(data: &[u32]) -> u32 {
    play(data, 30_000_000)
}
