fn item_priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => unreachable!(),
    }
}

fn parse_compartment(items: &[u8]) -> u64 {
    items
        .iter()
        .fold(0u64, |acc, &item| acc | 1u64 << item_priority(item))
}

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let (left, right) = bytes.split_at(bytes.len() / 2);
            (parse_compartment(left), parse_compartment(right))
        })
        .collect()
}

fn find_item(items: u64) -> u32 {
    (1..=52).find(|&item| items & (1 << item) != 0).unwrap()
}

pub fn part1(input: &[(u64, u64)]) -> u32 {
    input
        .iter()
        .map(|(left, right)| left & right)
        .fold(0, |sum, items| sum + find_item(items))
}

pub fn part2(input: &[(u64, u64)]) -> u32 {
    input
        .chunks(3)
        .map(|rucksacks| {
            rucksacks
                .iter()
                .map(|(left, right)| left | right)
                .reduce(|acc, rucksack| acc & rucksack)
                .unwrap_or(0)
        })
        .fold(0, |sum, items| sum + find_item(items))
}
