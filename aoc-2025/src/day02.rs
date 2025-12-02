use std::io::Write;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    start: u64,
    end: u64,
}

pub fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(",")
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            Range { start, end }
        })
        .collect()
}

fn sequence_repeated_twice(num: u64, buf: &mut Vec<u8>) -> bool {
    buf.clear();
    write!(buf, "{num}").unwrap();

    if buf.len().is_multiple_of(2) {
        let mut iter = buf.chunks(2);
        if let Some(first) = iter.next()
            && iter.all(|element| element == first)
        {
            return true;
        }
    }

    false
}

fn sequence_repeated(num: u64, buf: &mut Vec<u8>) -> bool {
    buf.clear();
    write!(buf, "{num}").unwrap();

    for i in 1..buf.len() {
        if !buf.len().is_multiple_of(i) {
            continue;
        }

        let mut iter = buf.chunks(i);
        if let Some(first) = iter.next()
            && iter.all(|element| element == first)
        {
            return true;
        }
    }
    false
}

pub fn part1(ranges: &[Range]) -> u64 {
    let mut buf = Vec::with_capacity(16);

    ranges
        .iter()
        .flat_map(|range| range.start..(range.end + 1))
        .filter(|&id| sequence_repeated_twice(id, &mut buf))
        .sum()
}

pub fn part2(ranges: &[Range]) -> u64 {
    let mut buf = Vec::with_capacity(16);

    ranges
        .iter()
        .flat_map(|range| range.start..(range.end + 1))
        .filter(|&id| sequence_repeated(id, &mut buf))
        .sum()
}
