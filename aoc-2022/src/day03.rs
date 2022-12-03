struct Store([bool; 52]);

impl Store {
    fn contains(&self, item: u8) -> bool {
        self.0[item as usize - 1]
    }
}

impl FromIterator<u8> for Store {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut store = [false; 52];
        for item in iter {
            store[item as usize - 1] = true;
        }
        Self(store)
    }
}

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

fn find_item(items: &[u8], store: &Store) -> u8 {
    *items.iter().find(|&&item| store.contains(item)).unwrap()
}

pub fn part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let store: Store = left.iter().copied().collect();

            (right, store)
        })
        .fold(0, |sum, (items, store)| {
            sum + find_item(items, &store) as u32
        })
}

pub fn part2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks(3)
        .map(|rucksacks| {
            let store: Store = rucksacks[0]
                .iter()
                .chain(rucksacks[1].iter())
                .copied()
                .collect();

            (&rucksacks[2], store)
        })
        .fold(0, |sum, (items, store)| {
            sum + find_item(items, &store) as u32
        })
}
