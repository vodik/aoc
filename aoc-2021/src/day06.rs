pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|segment| segment.parse().unwrap())
        .collect()
}

fn simulate(data: &[u64], generations: usize) -> usize {
    let mut tank = [0usize; 9];
    for &fish in data {
        tank[fish as usize] += 1;
    }

    for _ in 0..generations {
        tank.rotate_left(1);
        tank[6] += tank[8];
    }

    tank.into_iter().sum()
}

pub fn part1(input: &[u64]) -> usize {
    simulate(input, 80)
}

pub fn part2(input: &[u64]) -> usize {
    simulate(input, 256)
}
