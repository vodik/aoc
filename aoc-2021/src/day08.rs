type Pattern = u8;

#[derive(Debug)]
pub struct Entry {
    signal: Vec<Pattern>,
    output: Vec<Pattern>,
}

fn parse_segments(input: &str) -> Vec<Pattern> {
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
                .sum()
        })
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (signal, output) = line.split_once(" | ").unwrap();
            Entry {
                signal: parse_segments(signal),
                output: parse_segments(output),
            }
        })
        .collect()
}

pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .flat_map(|entry| &entry.output)
        .filter(|&pattern| matches!(pattern.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

struct DecipherKey(Pattern, Pattern);

impl DecipherKey {
    fn from(possibilities: &[Pattern]) -> Option<Self> {
        let &one = possibilities
            .iter()
            .find(|&pattern| pattern.count_ones() == 2)?;
        let &four = possibilities
            .iter()
            .find(|&pattern| pattern.count_ones() == 4)?;
        Some(Self(one, four))
    }

    fn decode(&self, pattern: Pattern) -> u8 {
        const PATTERN_LOOKUP: [u8; 16] = [4, 0, 1, 0, 0, 7, 9, 3, 0, 8, 0, 5, 0, 0, 6, 2];

        PATTERN_LOOKUP[(self.0 ^ pattern).count_ones() as usize * 2
            + (self.1 ^ pattern).count_ones() as usize * 2
            - pattern.count_ones() as usize]
    }
}

pub fn part2(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|Entry { signal, output }| {
            let key = DecipherKey::from(signal).unwrap();
            output
                .iter()
                .map(|&x| key.decode(x) as u32)
                .fold(0, |acc, value| acc * 10 + value)
        })
        .sum()
}
