pub fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|segment| segment.parse().unwrap())
        .collect()
}

fn simulate(data: &[u8], generations: usize) -> usize {
    let mut tank = [0; 9];
    for &fish in data {
        tank[fish as usize] += 1;
    }

    for _ in 0..(generations / 9) {
        tank[7] += tank[0];
        tank[8] += tank[1];
        tank[0] += tank[2];
        tank[1] += tank[3];
        tank[2] += tank[4];
        tank[3] += tank[5];
        tank[4] += tank[6];
        tank[5] += tank[7];
        tank[6] += tank[8];
    }

    for generation in 0..(generations % 9) {
        tank[(generation + 7) % 9] += tank[generation];
    }

    tank.iter().sum()
}

pub fn part1(input: &[u8]) -> usize {
    simulate(input, 80)
}

pub fn part2(input: &[u8]) -> usize {
    simulate(input, 256)
}
