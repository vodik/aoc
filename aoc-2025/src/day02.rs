use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct Range {
    start: u64,
    end: u64,
}

#[derive(Default)]
struct Buffer([u8; 20]);

impl Buffer {
    fn new() -> Self {
        Self::default()
    }

    fn set_value(&mut self, mut n: u64) -> usize {
        let mut i = 0;
        loop {
            self.0[i] = (n % 10) as u8;
            n /= 10;
            i += 1;
            if n == 0 {
                break;
            }
        }
        i
    }
}

impl Index<usize> for Buffer {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            Range { start, end }
        })
        .collect()
}

fn sequence_repeated_twice(num: u64, buf: &mut Buffer) -> bool {
    let len = buf.set_value(num);
    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    (0..mid).all(|i| buf[i] == buf[i + mid])
}

fn sequence_repeated(num: u64, buf: &mut Buffer) -> bool {
    let len = buf.set_value(num);

    (1..=len / 2)
        .filter(|&k| len.is_multiple_of(k))
        .any(|k| (k..len).all(|i| buf[i] == buf[i % k]))
}

pub fn part1(ranges: &[Range]) -> u64 {
    let mut buf = Buffer::new();

    ranges
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&id| sequence_repeated_twice(id, &mut buf))
        .sum()
}

pub fn part2(ranges: &[Range]) -> u64 {
    let mut buf = Buffer::new();

    ranges
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&id| sequence_repeated(id, &mut buf))
        .sum()
}
