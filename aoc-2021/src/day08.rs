#[derive(Debug)]
pub struct Entry {
    signal: Vec<Segments>,
    output: Vec<Segments>,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
type Segments = u8;
// pub struct Segment(u8);

fn parse2(input: &str) -> Vec<Segments> {
    input
        .split(' ')
        .map(|segment| {
            segment
                .as_bytes()
                .iter()
                .map(|b| match b {
                    b'a' => 1 << 0,
                    b'b' => 1 << 1,
                    b'c' => 1 << 2,
                    b'd' => 1 << 3,
                    b'e' => 1 << 4,
                    b'f' => 1 << 5,
                    b'g' => 1 << 6,
                    _ => unreachable!(),
                })
                .reduce(|acc, bit| acc | bit)
                .unwrap()
        })
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (signal, output) = line.split_once(" | ").unwrap();
            Entry {
                signal: parse2(signal),
                output: parse2(output),
            }
        })
        .collect()
}

pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .flat_map(|entry| &entry.output)
        .filter(|&segment| matches!(segment.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

pub fn map_digits(mut possibilities: Vec<Segments>) -> [Segments; 10] {
    let mut known = [0; 10];

    for (pos, count) in [(1, 2), (7, 3), (4, 4), (8, 7)] {
        known[pos] = possibilities.remove(
            possibilities
                .iter()
                .position(|x| x.count_ones() == count)
                .unwrap(),
        );
    }

    known[6] = possibilities.remove(
        possibilities
            .iter()
            .position(|x| x.count_ones() == 6 && (x & !known[1]).count_ones() == 5)
            .unwrap(),
    );

    known[0] = possibilities.remove(
        possibilities
            .iter()
            .position(|x| x.count_ones() == 6 && (x & !known[4]).count_ones() == 3)
            .unwrap(),
    );

    known[9] = possibilities.remove(
        possibilities
            .iter()
            .position(|x| x.count_ones() == 6)
            .unwrap(),
    );

    known[2] = possibilities.remove(
        possibilities
            .iter()
            .position(|e| (e & known[4]).count_ones() == 2)
            .unwrap(),
    );

    known[3] = possibilities.remove(
        possibilities
            .iter()
            .position(|e| (e & known[1]).count_ones() == 2)
            .unwrap(),
    );

    known[5] = possibilities.pop().unwrap();
    known
}

pub fn decode(entry: &Entry) -> u32 {
    let map = map_digits(entry.signal.clone());

    entry
        .output
        .iter()
        .map(|&x| map.iter().position(|&y| x == y).unwrap() as u32)
        .fold(0, |acc, value| acc * 10 + value)
}

pub fn part2(input: &[Entry]) -> u32 {
    input.iter().map(decode).sum()
}
