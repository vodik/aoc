const BITWIDTH: usize = 12;
const MASK: u16 = (1 << BITWIDTH) - 1;

pub fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn part1(input: &[u16]) -> u32 {
    let mut counts = [0usize; BITWIDTH];

    for num in input {
        for (shift, count) in counts.iter_mut().enumerate() {
            if num & 1 << shift != 0 {
                *count += 1;
            }
        }
    }

    let gamma = counts
        .iter()
        .rev()
        .map(|count| if count * 2 > input.len() { 1 } else { 0 })
        .fold(0u16, |gamma, bit| gamma << 1 | bit);

    gamma as u32 * (!gamma & MASK) as u32
}

fn scan(mut candidates: Vec<u16>, prefer_ones: bool) -> Option<u16> {
    for shift in 0..BITWIDTH {
        let mask = 1 << (BITWIDTH - shift - 1);
        let count = candidates.iter().filter(|&num| num & mask == 0).count();

        let expect_one = prefer_ones ^ (count * 2 < candidates.len());
        candidates.retain(|&num| expect_one ^ (num & mask == 0));

        if candidates.len() <= 1 {
            break;
        }
    }

    candidates.pop()
}

pub fn part2(input: &[u16]) -> u32 {
    let oxygen = scan(input.to_vec(), true).unwrap();
    let co2 = scan(input.to_vec(), false).unwrap();

    oxygen as u32 * co2 as u32
}
