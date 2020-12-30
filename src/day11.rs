use crate::parsers::grid;
use nom::{
    combinator::{all_consuming, map},
    error::Error,
    Finish, IResult,
};
use std::{
    collections::hash_map::DefaultHasher,
    convert::TryInto,
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Hash)]
struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}

impl Grid {
    fn get_index(&self, &(x, y): &(i32, i32)) -> usize {
        (y * self.width + x).try_into().unwrap()
    }

    fn get(&self, coord @ (x, y): &(i32, i32)) -> Option<&Cell> {
        if (0..self.width).contains(x) && (0..self.height).contains(y) {
            self.cells.get(self.get_index(coord))
        } else {
            None
        }
    }

    fn neighbour_count(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;

        for &delta_x in &[-1, 0, 1] {
            for &delta_y in &[-1, 0, 1] {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }

                if let Some(Cell::Occupied) = self.get(&(x + delta_x, y + delta_y)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn neighbours_seen(&self, x: i32, y: i32) -> u8 {
        let ray_trace = |coord| match self.get(&coord) {
            Some(&Cell::Occupied) => Some(1),
            Some(&Cell::Empty) => Some(0),
            Some(&Cell::Floor) => None,
            None => Some(0),
        };

        [
            (x + 1..).find_map(|x| ray_trace((x, y))).unwrap_or(0),
            (y + 1..).find_map(|y| ray_trace((x, y))).unwrap_or(0),
            (x + 1..)
                .zip(y + 1..)
                .find_map(|(x, y)| ray_trace((x, y)))
                .unwrap_or(0),
            (x + 1..)
                .zip((0..y).rev())
                .find_map(|(x, y)| ray_trace((x, y)))
                .unwrap_or(0),
            (0..x).rev().find_map(|x| ray_trace((x, y))).unwrap_or(0),
            (0..y).rev().find_map(|y| ray_trace((x, y))).unwrap_or(0),
            (0..x)
                .rev()
                .zip(y + 1..)
                .find_map(|(x, y)| ray_trace((x, y)))
                .unwrap_or(0),
            (0..x)
                .rev()
                .zip((0..y).rev())
                .find_map(|(x, y)| ray_trace((x, y)))
                .unwrap_or(0),
        ]
        .iter()
        .sum()
    }

    fn next_generation(&mut self) {
        let mut next = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = next.get_mut(self.get_index(&(x, y))).unwrap();
                let neighbors = self.neighbour_count(x, y);

                *cell = match (*cell, neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, x) if x >= 4 => Cell::Empty,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = next;
    }

    fn next_generation_with_ray_tracing(&mut self) {
        let mut next = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = next.get_mut(self.get_index(&(x, y))).unwrap();
                let neighbors = self.neighbours_seen(x, y);

                *cell = match (*cell, neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, x) if x >= 5 => Cell::Empty,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = next;
    }
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    map(grid("#L."), |(grid, (width, height))| {
        let cells = grid
            .into_iter()
            .map(|b| match b {
                b'.' => Cell::Floor,
                b'L' => Cell::Empty,
                b'#' => Cell::Occupied,
                _ => unreachable!(),
            })
            .collect();

        Grid {
            cells,
            width: width as i32,
            height: height as i32,
        }
    })(input)
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Result<Grid, Error<String>> {
    match all_consuming(parse_grid)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[aoc(day11, part1)]
fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut hash = calculate_hash(&grid);

    loop {
        grid.next_generation();
        let new_hash = calculate_hash(&grid);
        if hash == new_hash {
            break;
        }
        hash = new_hash;
    }

    grid.cells
        .iter()
        .filter(|&cell| cell == &Cell::Occupied)
        .count()
}

#[aoc(day11, part2)]
fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut hash = calculate_hash(&grid);

    loop {
        grid.next_generation_with_ray_tracing();
        let new_hash = calculate_hash(&grid);
        if hash == new_hash {
            break;
        }
        hash = new_hash;
    }

    grid.cells
        .iter()
        .filter(|&cell| cell == &Cell::Occupied)
        .count()
}
