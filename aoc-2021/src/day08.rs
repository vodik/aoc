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

struct DecipherKey([u8; 1 << 7]);

impl DecipherKey {
    fn from(possibilities: &[Pattern]) -> Self {
        let mut one = 0;
        let mut four = 0;

        for &pattern in possibilities {
            match pattern.count_ones() {
                2 => {
                    one = pattern;
                }
                4 => {
                    four = pattern;
                }
                _ => {}
            }
        }

        let mut map = [0; 1 << 7];
        for &pattern in possibilities {
            map[pattern as usize] = match pattern.count_ones() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 if (pattern & one).count_ones() == 2 => 3,
                5 if (pattern & four).count_ones() == 2 => 2,
                5 => 5,
                6 if (pattern & !one).count_ones() == 5 => 6,
                6 if (pattern & !four).count_ones() == 3 => 0,
                6 => 9,
                7 => 8,
                _ => unreachable!(),
            };
        }
        Self(map)
    }

    fn decode(&self, pattern: Pattern) -> u8 {
        self.0[pattern as usize]
    }
}

pub fn part2(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|Entry { signal, output }| {
            let key = DecipherKey::from(signal);
            output
                .iter()
                .map(|&x| key.decode(x) as u32)
                .fold(0, |acc, value| acc * 10 + value)
        })
        .sum()
}
