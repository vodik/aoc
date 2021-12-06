pub fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|segment| segment.parse().unwrap())
        .collect()
}

fn simulate<const N: usize>(data: &[u8]) -> usize {
    let mut tank = [0; 9];
    for &fish in data {
        tank[fish as usize] += 1;
    }

    const BUMP: [usize; 9] = [7, 8, 0, 1, 2, 3, 4, 5, 6];
    for _ in 0..(N / 9) {
        for (i, &j) in BUMP.iter().enumerate() {
            tank[j] += tank[i];
        }
    }

    for generation in 0..(N % 9) {
        tank[(generation + 7) % 9] += tank[generation];
    }

    tank.iter().sum()
}

pub fn part1(input: &[u8]) -> usize {
    simulate::<80>(input)
}

pub fn part2(input: &[u8]) -> usize {
    simulate::<256>(input)
}
