use std::{cmp::Reverse, iter, num::NonZeroUsize};

pub type Grid = (Vec<u8>, usize);

pub fn parse_input(input: &str) -> Grid {
    let mut width: Option<NonZeroUsize> = None;
    let grid: Vec<u8> = input
        .lines()
        .flat_map(|line| {
            if width.is_none() {
                width = NonZeroUsize::new(line.len());
            }

            line.chars().map(|c| c.to_digit(10).unwrap() as u8)
        })
        .collect();

    (grid, width.unwrap().get())
}

fn neighbours(grid: &Grid, pos: usize) -> impl Iterator<Item = (usize, u8)> + '_ {
    let mut kind = 0;
    iter::from_fn(move || {
        if kind == 0 {
            kind += 1;
            if pos % grid.1 != 0 {
                return Some(pos - 1);
            }
        }
        if kind == 1 {
            kind += 1;
            if pos % grid.1 != grid.1 - 1 {
                return Some(pos + 1);
            }
        }
        if kind == 2 {
            kind += 1;
            return Some(pos + grid.1);
        }
        if kind == 3 {
            kind += 1;
            return Some(pos - grid.1);
        }
        None
    })
    .flat_map(|pos| grid.0.get(pos).map(|&cell| (pos, cell)))
}

pub fn part1(input @ (map, _): &Grid) -> usize {
    map.iter()
        .enumerate()
        .filter(|&(pos, &cell)| neighbours(input, pos).all(|(_, neighbour)| neighbour > cell))
        .map(|(_, &cell)| cell as usize + 1)
        .sum()
}

pub fn part2(input: &Grid) -> usize {
    let mut basins: Vec<usize> = Vec::new();
    let mut visited = vec![false; input.0.len()];

    for (pos, &cell) in input.0.iter().enumerate() {
        if visited[pos] || cell == 9 {
            continue;
        }

        let mut basin = 0;
        let mut stack = vec![pos];

        while let Some(pos) = stack.pop() {
            if visited[pos] {
                continue;
            }

            visited[pos] = true;
            basin += 1;

            stack.extend(
                neighbours(input, pos)
                    .filter_map(|(neighbour, cell)| (cell != 9).then(|| neighbour)),
            );
        }

        basins.push(basin);
    }

    basins.sort_unstable_by_key(|&k| Reverse(k));
    basins[0] * basins[1] * basins[2]
}
