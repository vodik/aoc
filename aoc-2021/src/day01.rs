pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn decent_step_count(size: usize, data: &[u32]) -> usize {
    data.windows(size)
        .filter(|&window| window[0] < window[size - 1])
        .count()
}

pub fn part1(input: &[u32]) -> usize {
    decent_step_count(2, input)
}

pub fn part2(input: &[u32]) -> usize {
    decent_step_count(4, input)
}
