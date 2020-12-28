use crate::conway;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Cell {
    Inactive,
    Active,
}

#[derive(Debug, Clone, Hash)]
struct Grid {
    width: usize,
    cells: Vec<Cell>,
}

#[aoc_generator(day17)]
fn parse_grid(input: &str) -> Grid {
    let mut width = None;

    let cells = input
        .lines()
        .flat_map(|line| {
            if let Some(width) = width {
                assert_eq!(width, line.len())
            } else {
                width = Some(line.len());
            }

            line.chars().map(|c| match c {
                '.' => Cell::Inactive,
                '#' => Cell::Active,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();

    Grid {
        cells,
        width: width.unwrap(),
    }
}

#[aoc(day17, part1)]
fn part1(grid: &Grid) -> usize {
    let mut board = grid
        .cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| {
            if cell == &Cell::Active {
                Some((
                    i32::try_from(idx % grid.width).unwrap(),
                    i32::try_from(idx / grid.width).unwrap(),
                    0i32,
                ))
            } else {
                None
            }
        })
        .collect::<conway::Board<_>>();

    for _ in 0..6 {
        board = board.next_generation();
    }

    board.alive_count()
}

#[aoc(day17, part2)]
fn part2(grid: &Grid) -> usize {
    let mut board = grid
        .cells
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| {
            if cell == &Cell::Active {
                Some((
                    i32::try_from(idx % grid.width).unwrap(),
                    i32::try_from(idx / grid.width).unwrap(),
                    0i32,
                    0i32,
                ))
            } else {
                None
            }
        })
        .collect::<conway::Board<_>>();

    for _ in 0..6 {
        board = board.next_generation();
    }

    board.alive_count()
}
