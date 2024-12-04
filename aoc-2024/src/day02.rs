pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(grid: &[Vec<i32>]) -> usize {
    grid.iter()
        .filter(|line| {
            let expected = line.len() as i32 - 1;
            let safe = line
                .windows(2)
                .map(|window| match window[1] - window[0] {
                    -3..=-1 => -1,
                    1..=3 => 1,
                    _ => 0,
                })
                .sum::<i32>()
                .abs();

            safe == expected
        })
        .count()
}

pub fn part2(grid: &[Vec<i32>]) -> i32 {
    let mut matches = 0;
    'outer: for line in grid {
        let expected = line.len() as i32 - 1;
        let safe = line
            .windows(2)
            .map(|window| match window[1] - window[0] {
                -3..=-1 => -1,
                1..=3 => 1,
                _ => 0,
            })
            .sum::<i32>()
            .abs();

        if safe == expected {
            matches += 1;
            continue;
        }

        for i in 0..line.len() {
            let expected = expected - 1;
            let mut line = line.clone();
            line.remove(i);

            let safe = line
                .windows(2)
                .map(|window| match window[1] - window[0] {
                    -3..=-1 => -1,
                    1..=3 => 1,
                    _ => 0,
                })
                .sum::<i32>()
                .abs();

            if safe == expected {
                matches += 1;
                continue 'outer;
            }
        }
    }
    matches
}
