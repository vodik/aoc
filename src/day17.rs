use crate::{conway, parsers::grid};
use nom::{
    combinator::{all_consuming, map},
    error::Error,
    Finish, IResult,
};
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Cell {
    Inactive,
    Active,
}

#[derive(Debug, Clone, Hash)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    map(grid("#."), |(grid, (width, _))| {
        let cells = grid
            .into_iter()
            .map(|b| match b {
                b'#' => Cell::Active,
                b'.' => Cell::Inactive,
                _ => unreachable!(),
            })
            .collect();

        Grid { cells, width }
    })(input)
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Result<Grid, Error<String>> {
    match all_consuming(parse_grid)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day17, part1)]
fn part1(grid: &Grid) -> usize {
    let board = grid
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
        .collect();

    conway::game_of_life(board, 6).alive_count()
}

#[aoc(day17, part2)]
fn part2(grid: &Grid) -> usize {
    let board = grid
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
        .collect();

    conway::game_of_life(board, 6).alive_count()
}
