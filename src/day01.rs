pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn decent_step_count(data: &[u32]) -> usize {
    data.windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

pub fn part1(input: &[u32]) -> usize {
    decent_step_count(input)
}

pub fn part2(input: &[u32]) -> usize {
    let windows: Vec<u32> = input.windows(3).map(|window| window.iter().sum()).collect();
    decent_step_count(&windows)
}
