use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect()
}

fn even_digits(n: u64) -> Option<(u64, u64)> {
    match n {
        10..100 => Some((n / 10, n % 10)),
        1000..10000 => Some((n / 100, n % 100)),
        100000..1000000 => Some((n / 1000, n % 1000)),
        10000000..100000000 => Some((n / 10000, n % 10000)),
        1000000000..10000000000 => Some((n / 100000, n % 100000)),
        100000000000..1000000000000 => Some((n / 1000000, n % 1000000)),
        10000000000000..100000000000000 => Some((n / 10000000, n % 10000000)),
        1000000000000000..10000000000000000 => Some((n / 100000000, n % 100000000)),
        100000000000000000..1000000000000000000 => Some((n / 1000000000, n % 1000000000)),
        10000000000000000000..=u64::MAX => Some((n / 10000000000, n % 10000000000)),
        _ => None,
    }
}

fn replace(stone: u64) -> [Option<u64>; 2] {
    if stone == 0 {
        [Some(1), None]
    } else if let Some((left, right)) = even_digits(stone) {
        [Some(left), Some(right)]
    } else {
        [Some(stone * 2024), None]
    }
}

fn blink(stones: HashMap<u64, u64>, buffer: &mut HashMap<u64, u64>) -> HashMap<u64, u64> {
    buffer.clear();
    for (&stone, &count) in stones.iter() {
        for &stone in replace(stone).iter().flatten() {
            *buffer.entry(stone).or_default() += count;
        }
    }
    std::mem::replace(buffer, stones)
}

pub fn part1(input: &[u64]) -> u64 {
    let stones: HashMap<u64, u64> = input.iter().map(|&stone| (stone, 1)).collect();
    let mut buffer = HashMap::with_capacity(stones.len());
    let stones = (0..25).fold(stones, |acc, _| blink(acc, &mut buffer));
    stones.values().sum()
}

pub fn part2(input: &[u64]) -> u64 {
    let stones: HashMap<u64, u64> = input.iter().map(|&stone| (stone, 1)).collect();
    let mut buffer = HashMap::with_capacity(stones.len());
    let stones = (0..75).fold(stones, |acc, _| blink(acc, &mut buffer));
    stones.values().sum()
}
