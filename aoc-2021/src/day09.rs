use std::{collections::BinaryHeap, num::NonZeroUsize};

#[derive(Debug)]
pub struct Grid(Vec<u8>, usize);

impl Grid {
    fn new(map: &[u8], width: usize) -> Self {
        let height = map.len() / width;

        let new_width = width + 2;
        let new_height = height + 2;
        let mut new_map = vec![9; new_width * new_height];

        let offset = new_width + 1;
        for (line, chunk) in map.chunks(width).enumerate() {
            let pos = line * new_width + offset;
            new_map[pos..pos + width].copy_from_slice(chunk);
        }

        Self(new_map, new_width)
    }

    fn neighbours(&self, pos: usize) -> impl Iterator<Item = (usize, u8)> + '_ {
        [pos - 1, pos + 1, pos + self.1, pos - self.1]
            .into_iter()
            .flat_map(|pos| self.0.get(pos).map(|&cell| (pos, cell)))
    }
}

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

    Grid::new(&grid, width.unwrap().get())
}

pub fn part1(grid: &Grid) -> usize {
    grid.0
        .iter()
        .enumerate()
        .filter(|&(_, &cell)| cell != 9)
        .filter(|&(pos, &cell)| grid.neighbours(pos).all(|(_, neighbour)| neighbour > cell))
        .map(|(_, &cell)| cell as usize + 1)
        .sum()
}

pub fn part2(grid: &Grid) -> usize {
    let mut basins: BinaryHeap<_> = grid
        .0
        .iter()
        .enumerate()
        .filter(|&(_, &cell)| cell != 9)
        .scan(
            (vec![false; grid.0.len()], Vec::with_capacity(100)),
            |(visited, stack), (pos, _)| {
                let mut basin = 0;

                if !visited[pos] {
                    stack.push(pos);
                    while let Some(pos) = stack.pop() {
                        if visited[pos] {
                            continue;
                        }

                        visited[pos] = true;
                        basin += 1;
                        stack.extend(
                            grid.neighbours(pos)
                                .filter_map(|(neighbour, cell)| (cell != 9).then(|| neighbour)),
                        );
                    }
                }

                Some(basin)
            },
        )
        .filter(|&basin| basin > 0)
        .collect();

    (0..3).flat_map(|_| basins.pop()).product()
}
