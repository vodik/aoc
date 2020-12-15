use std::collections::HashMap;
use std::num::ParseIntError;

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

    let mut current: u32 = 0;
    for turn in data.len() + 1..target {
        current = match history.insert(current, turn) {
            Some(previous) => (turn - previous) as u32,
            None => 0,
        }
    }

    current
}

#[aoc(day15, part1)]
fn part1(data: &[u32]) -> u32 {
    play(data, 2020)
}

#[aoc(day15, part2)]
fn part2(data: &[u32]) -> u32 {
    play(data, 30_000_000)
}
