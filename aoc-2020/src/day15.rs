use std::num::ParseIntError;

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

fn play(data: &[u32], target: u32) -> u32 {
    let mut history = vec![0u32; target as usize];

    for (turn, &value) in data.iter().enumerate() {
        history[value as usize] = turn as u32 + 1;
    }

    let last = *data.last().unwrap();
    (data.len() as u32..target).fold(last, |last, turn| {
        let entry = &mut history[last as usize];

        let previously = *entry;
        *entry = turn;
        if previously == 0 {
            0
        } else {
            turn - previously
        }
    })
}

#[aoc(day15, part1)]
fn part1(data: &[u32]) -> u32 {
    play(data, 2020)
}

#[aoc(day15, part2)]
fn part2(data: &[u32]) -> u32 {
    play(data, 30_000_000)
}
