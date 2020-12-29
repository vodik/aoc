use crate::parsers::grid;
use nom::{
    combinator::{all_consuming, map},
    error::Error,
    Finish, IResult,
};

#[derive(Debug, PartialEq)]
enum Tile {
    Tree,
    Empty,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    fn path(&self, (right, down): (usize, usize)) -> usize {
        (0..)
            .map(|step| {
                self.tiles
                    .get(step * down * self.width + step * right % self.width)
            })
            .take_while(|tile| tile.is_some())
            .filter(|tile| tile.unwrap() == &Tile::Tree)
            .count()
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    map(grid("#."), |(grid, (width, _))| {
        let tiles = grid
            .into_iter()
            .map(|b| match b {
                b'#' => Tile::Tree,
                b'.' => Tile::Empty,
                _ => unreachable!(),
            })
            .collect();

        Map { tiles, width }
    })(input)
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Result<Map, Error<String>> {
    match all_consuming(parse_map)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    map.path((3, 1))
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&slope| map.path(slope))
        .product()
}
