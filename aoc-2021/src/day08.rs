use std::collections::HashSet;

#[derive(Debug)]
pub struct Entry {
    signal: Vec<HashSet<Segment>>,
    output: Vec<HashSet<Segment>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Segment(u8);

fn parse2(input: &str) -> Vec<HashSet<Segment>> {
    input
        .split(' ')
        .map(|segment| {
            segment
                .as_bytes()
                .iter()
                .map(|b| match b {
                    b'a' => Segment(1),
                    b'b' => Segment(2),
                    b'c' => Segment(3),
                    b'd' => Segment(4),
                    b'e' => Segment(5),
                    b'f' => Segment(6),
                    b'g' => Segment(7),
                    _ => unreachable!(),
                })
                .collect()
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
        .filter(|&segment| [2, 3, 4, 7].contains(&segment.len()))
        .count()
}

pub fn map_digits(mut init_pos: Vec<HashSet<Segment>>) -> Vec<HashSet<Segment>> {
    let mut true_pos = vec![init_pos[0].clone(); 10];

    for uniq in [(1, 2), (7, 3), (4, 4), (8, 7)] {
        true_pos[uniq.0] =
            init_pos.remove(init_pos.iter().position(|x| x.len() == uniq.1).unwrap());
    }

    true_pos[6] = init_pos.remove(
        init_pos
            .iter()
            .position(|x| x.len() == 6 && x.difference(&true_pos[1]).count() == 5)
            .unwrap(),
    );

    true_pos[0] = init_pos.remove(
        init_pos
            .iter()
            .position(|x| x.len() == 6 && x.difference(&true_pos[4]).count() == 3)
            .unwrap(),
    );

    true_pos[9] = init_pos.remove(init_pos.iter().position(|x| x.len() == 6).unwrap());

    true_pos[2] = init_pos.remove(
        init_pos
            .iter()
            .position(|e| e.intersection(&true_pos[4]).count() == 2)
            .unwrap(),
    );

    true_pos[3] = init_pos.remove(
        init_pos
            .iter()
            .position(|e| e.intersection(&true_pos[1]).count() == 2)
            .unwrap(),
    );

    true_pos[5] = init_pos.pop().unwrap();
    true_pos
}

pub fn decode(entry: &Entry) -> u32 {
    let init_pos: Vec<HashSet<Segment>> = entry.signal.clone();

    let map = map_digits(init_pos);

    entry
        .output
        .iter()
        .map(|x| x.iter().copied().collect::<HashSet<_>>())
        .map(|x| map.iter().position(|y| &x == y).unwrap() as u32)
        .fold(0, |acc, value| acc * 10 + value)
}

pub fn part2(input: &[Entry]) -> u32 {
    input.iter().map(decode).sum()
}
