struct Inventory(u64);

impl Inventory {
    fn contains(&self, item: u8) -> bool {
        self.0 & (1 << item) != 0
    }
}

impl FromIterator<u8> for Inventory {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().fold(0u64, |store, item| store | 1 << item))
    }
}

fn item_priority(item: u8) -> u8 {
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

fn find_item(items: &[u8], inventory: &Inventory) -> u8 {
    *items
        .iter()
        .find(|&&item| inventory.contains(item))
        .unwrap()
}

pub fn part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let inventory: Inventory = left.iter().copied().collect();

            (right, inventory)
        })
        .fold(0, |sum, (items, inventory)| {
            sum + find_item(items, &inventory) as u32
        })
}

pub fn part2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks(3)
        .map(|rucksacks| {
            let inventory: Inventory = rucksacks[0]
                .iter()
                .chain(rucksacks[1].iter())
                .copied()
                .collect();

            (&rucksacks[2], inventory)
        })
        .fold(0, |sum, (items, inventory)| {
            sum + find_item(items, &inventory) as u32
        })
}
