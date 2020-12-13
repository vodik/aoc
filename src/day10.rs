use std::collections::HashMap;
use std::num::ParseIntError;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    let mut data = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    data.push(0);
    data.sort_unstable();
    data.push(*data.last().unwrap() + 3);
    Ok(data)
}

#[aoc(day10, part1)]
fn part1_alt(data: &[u64]) -> usize {
    let mut jolt1 = 0;
    let mut jolt3 = 0;

    for delta in data.windows(2).map(|pairs| pairs[1] - pairs[0]) {
        match delta {
            1 => jolt1 += 1,
            2 => {}
            3 => jolt3 += 1,
            _ => panic!(),
        }
    }

    jolt1 * jolt3
}

fn arrange(data: &[u64], idx: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(hit) = cache.get(&idx) {
        return *hit;
    }

    if let Some(j1) = data.get(idx) {
        let mut result = arrange(data, idx + 1, cache);

        if let Some(j2) = data.get(idx + 2) {
            if *j2 - *j1 <= 3 {
                result += arrange(data, idx + 2, cache);

                if let Some(j3) = data.get(idx + 3) {
                    if *j3 - *j1 <= 3 {
                        result += arrange(data, idx + 3, cache);
                    }
                }
            }
        }

        cache.insert(idx, result);
        result
    } else {
        1
    }
}

#[aoc(day10, part2)]
fn part2(data: &[u64]) -> usize {
    arrange(&data, 0, &mut Default::default())
}
