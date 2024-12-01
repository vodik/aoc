pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_set = vec![];
    let mut right_set = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        left_set.push(left.parse().unwrap());
        right_set.push(right.parse().unwrap());
    }

    left_set.sort_unstable();
    right_set.sort_unstable();
    (left_set, right_set)
}

pub fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub(crate) fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let min = *right.first().unwrap();
    let max = *right.last().unwrap();

    let frequency = right
        .iter()
        .fold(vec![0u8; (max - min + 1) as _], |mut acc, value| {
            acc[(value - min) as usize] += 1;
            acc
        });

    left.iter()
        .filter(|&&value| value >= min && value <= max)
        .map(|value| value * frequency[(value - min) as usize] as u32)
        .sum()
}
