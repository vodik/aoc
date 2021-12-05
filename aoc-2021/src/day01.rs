pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn decent_step_count<const N: usize>(data: &[u32]) -> usize {
    data.windows(N)
        .filter(|&window| window[0] < window[N - 1])
        .count()
}

pub fn part1(input: &[u32]) -> usize {
    decent_step_count::<2>(input)
}

pub fn part2(input: &[u32]) -> usize {
    decent_step_count::<4>(input)
}
