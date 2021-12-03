use crate::conway::{self, Neighbors};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    error::Error,
    multi::{many1, separated_list1},
    Finish, IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Tile {
    Black,
    White,
}

impl Tile {
    fn flip(&mut self) {
        *self = match self {
            Tile::Black => Tile::White,
            Tile::White => Tile::Black,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::White
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Axial(i32, i32);

impl Axial {
    fn step(self, direction: &Direction) -> Self {
        let Axial(q, r) = self;
        match direction {
            Direction::East => Axial(q + 1, r),
            Direction::SouthEast => Axial(q, r + 1),
            Direction::SouthWest => Axial(q - 1, r + 1),
            Direction::West => Axial(q - 1, r),
            Direction::NorthWest => Axial(q, r - 1),
            Direction::NorthEast => Axial(q + 1, r - 1),
        }
    }
}

impl Default for Axial {
    fn default() -> Self {
        Self(0, 0)
    }
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("e"), |_| Direction::East),
        map(tag("se"), |_| Direction::SouthEast),
        map(tag("sw"), |_| Direction::SouthWest),
        map(tag("w"), |_| Direction::West),
        map(tag("nw"), |_| Direction::NorthWest),
        map(tag("ne"), |_| Direction::NorthEast),
    ))(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    separated_list1(tag("\n"), many1(direction))(input)
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Result<Vec<Vec<Direction>>, Error<String>> {
    match all_consuming(parse_directions)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn lay_tiles(data: &[Vec<Direction>]) -> HashMap<Axial, Tile> {
    let mut map: HashMap<Axial, Tile> = HashMap::new();

    for instruction in data {
        let point = instruction
            .iter()
            .fold(Axial::default(), |point, step| point.step(&step));
        map.entry(point).or_default().flip();
    }

    map
}

#[aoc(day24, part1)]
fn part1(data: &[Vec<Direction>]) -> usize {
    lay_tiles(data)
        .into_values()
        .filter(|&tile| tile == Tile::Black)
        .count()
}

impl Neighbors for Axial {
    fn neighbours(&self) -> Vec<Self> {
        vec![
            self.step(&Direction::East),
            self.step(&Direction::SouthEast),
            self.step(&Direction::SouthWest),
            self.step(&Direction::West),
            self.step(&Direction::NorthWest),
            self.step(&Direction::NorthEast),
        ]
    }

    fn activate(active: bool, neighbours: usize) -> bool {
        matches!((active, neighbours), (true, 1) | (true, 2) | (false, 2))
    }
}

#[aoc(day24, part2)]
fn part2(data: &[Vec<Direction>]) -> usize {
    let board = lay_tiles(data)
        .into_iter()
        .filter_map(|(point, tile)| {
            if tile == Tile::Black {
                Some(point)
            } else {
                None
            }
        })
        .collect();

    conway::game_of_life(board, 100).alive_count()
}
