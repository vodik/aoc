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
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn step(&mut self, direction: &Direction) -> &Self {
        match direction {
            Direction::East => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::SouthEast => {
                self.y -= 1;
                self.z += 1;
            }
            Direction::SouthWest => {
                self.x -= 1;
                self.z += 1;
            }
            Direction::West => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::NorthWest => {
                self.y += 1;
                self.z -= 1;
            }
            Direction::NorthEast => {
                self.x += 1;
                self.z -= 1;
            }
        }
        self
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0, 0, 0)
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

fn lay_tiles(data: &[Vec<Direction>]) -> HashMap<Point, Tile> {
    let mut map: HashMap<Point, Tile> = HashMap::new();

    for instruction in data {
        let mut point = Point::default();
        for step in instruction {
            point.step(&step);
        }

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

impl Neighbors for Point {
    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = Vec::with_capacity(6);

        for &x_delta in &[-1, 0, 1] {
            for &y_delta in &[-1, 0, 1] {
                for &z_delta in &[-1, 0, 1] {
                    if (x_delta == 0 && y_delta == 0 && z_delta == 0)
                        || (x_delta + y_delta + z_delta != 0)
                    {
                        continue;
                    }

                    neighbours.push(Point::new(
                        self.x + x_delta,
                        self.y + y_delta,
                        self.z + z_delta,
                    ));
                }
            }
        }

        neighbours
    }

    fn activate(active: bool, neighbours: usize) -> bool {
        matches!((active, neighbours), (true, 1) | (true, 2) | (false, 2))
    }
}

#[aoc(day24, part2)]
fn part2(data: &[Vec<Direction>]) -> usize {
    let mut board = lay_tiles(data)
        .into_iter()
        .filter_map(|(point, tile)| {
            if tile == Tile::Black {
                Some(point)
            } else {
                None
            }
        })
        .collect::<conway::Board<_>>();

    for _ in 0..100 {
        board = board.next_generation();
    }

    board.alive_count()
}
