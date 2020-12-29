use std::{collections::HashSet, num::ParseIntError};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    let mut data = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    data.push(0);
    data.sort_unstable();
    data.push(*data.last().unwrap() + 3);
    Ok(data)
}

#[aoc(day10, part1)]
fn part1_alt(data: &[u64]) -> usize {
    let mut jolt1 = 0;
    let mut jolt3 = 0;

    for delta in data.windows(2).map(|pairs| pairs[1] - pairs[0]) {
        match delta {
            1 => jolt1 += 1,
            2 => {}
            3 => jolt3 += 1,
            _ => panic!("Gap in ratings larger than 3"),
        }
    }

    jolt1 * jolt3
}

#[aoc(day10, part2)]
fn part2_dynamic(data: &[u64]) -> u64 {
    let adapters: HashSet<u64> = data.iter().copied().collect();

    let max = *data.last().unwrap() as usize;
    let mut combinations: Vec<u64> = vec![0; max as usize + 2];
    combinations[max] = 1;

    for idx in (0..max).rev() {
        if adapters.contains(&(idx as u64)) {
            combinations[idx] += combinations[idx + 1];
            combinations[idx] += combinations[idx + 2];
            combinations[idx] += combinations[idx + 3];
        }
    }

    combinations[0]
}
