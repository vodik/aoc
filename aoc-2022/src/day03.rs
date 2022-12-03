pub fn item_priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => unreachable!(),
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&item| item_priority(item))
                .collect()
        })
        .collect()
}

fn find_item(items: &[u8], store: &[bool; 52]) -> u8 {
    *items
        .iter()
        .find(|&&item| store[item as usize - 1])
        .unwrap()
}

pub fn part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);

            let mut store = [false; 52];
            for &item in left {
                store[item as usize - 1] = true;
            }

            (store, right)
        })
        .fold(0, |sum, (store, items)| {
            sum + find_item(items, &store) as u32
        })
}

pub fn part2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks(3)
        .map(|rucksacks| {
            let mut store = [false; 52];
            for &item in rucksacks[0].iter().chain(rucksacks[1].iter()) {
                store[item as usize - 1] = true;
            }

            (store, &rucksacks[2])
        })
        .fold(0, |sum, (store, items)| {
            sum + find_item(items, &store) as u32
        })
}
