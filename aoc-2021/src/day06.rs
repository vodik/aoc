pub fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|segment| segment.parse().unwrap())
        .collect()
}

fn simulate(data: &[u8], generations: usize) -> usize {
    let mut tank = [0usize; 9];
    for &fish in data {
        tank[fish as usize] += 1;
    }

    for generation in 0..generations {
        let base = generation % 9;
        tank[(base + 7) % 9] += tank[base];
    }

    tank.into_iter().sum()
}

pub fn part1(input: &[u8]) -> usize {
    simulate(input, 80)
}

pub fn part2(input: &[u8]) -> usize {
    simulate(input, 256)
}
