use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    error::Error,
    multi::{many1, separated_list1},
    Finish, IResult,
};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
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

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn step(&mut self, direction: &Direction) {
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

#[aoc(day24, part1)]
fn part1(data: &[Vec<Direction>]) -> usize {
    let mut map: HashMap<Point, Tile> = HashMap::new();

    for instruction in data {
        let mut point = Point::default();
        for step in instruction {
            point.step(&step);
        }

        map.entry(point).or_default().flip();
    }

    map.values().filter(|&tile| tile == &Tile::Black).count()
}

#[derive(Debug, Clone, Hash)]
struct Grid {
    width: i32,
    cells: Vec<Tile>,
}

impl Grid {
    fn new(cells: Vec<Tile>, width: i32) -> Self {
        Grid { cells, width }
    }

    fn get_index(&self, x: i32, y: i32, z: i32) -> usize {
        usize::try_from(z * self.width * self.width + y * self.width + x).unwrap()
    }

    fn neighbour_count(&self, x: i32, y: i32, z: i32) -> u8 {
        let mut count = 0;
        for x_delta in [-1, 0, 1].iter().cloned() {
            for y_delta in [-1, 0, 1].iter().cloned() {
                for z_delta in [-1, 0, 1].iter().cloned() {
                    if x_delta == 0 && y_delta == 0 && z_delta == 0 {
                        continue;
                    }
                    if x_delta + y_delta + z_delta != 0 {
                        continue;
                    }

                    let neighbour_x = x + x_delta;
                    if !(0..self.width).contains(&neighbour_x) {
                        continue;
                    }

                    let neighbour_y = y + y_delta;
                    if !(0..self.width).contains(&neighbour_y) {
                        continue;
                    }

                    let neighbour_z = z + z_delta;
                    if !(0..self.width).contains(&neighbour_z) {
                        continue;
                    }

                    let idx = self.get_index(neighbour_x, neighbour_y, neighbour_z);
                    if self.cells[idx] == Tile::Black {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next: Vec<Tile> = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.width {
                for z in 0..self.width {
                    let idx = self.get_index(x, y, z);
                    let cell = self.cells[idx];
                    let neighbors = self.neighbour_count(x, y, z);

                    let next_cell = match (cell, neighbors) {
                        (Tile::White, 2) => Tile::Black,
                        (Tile::Black, x) if x == 0 || x > 2 => Tile::White,
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cell;
                }
            }
        }

        self.cells = next;
    }
}

#[aoc(day24, part2)]
fn part2(data: &[Vec<Direction>]) -> usize {
    let size = 140;
    let mut map = vec![Tile::default(); size * size * size];

    // Build initial grid
    for instruction in data {
        let mut point = Point::new(70, 70, 70);
        for step in instruction {
            point.step(&step);
        }

        map.get_mut(
            usize::try_from(point.z).unwrap() * size * size
                + usize::try_from(point.y).unwrap() * size
                + usize::try_from(point.x).unwrap(),
        )
        .unwrap()
        .flip();
    }

    let mut grid = Grid::new(map, i32::try_from(size).unwrap());
    for _ in 0..100 {
        grid.tick();
    }

    grid.cells
        .iter()
        .filter(|&tile| tile == &Tile::Black)
        .count()
}
