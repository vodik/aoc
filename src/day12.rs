use crate::parsers::number;
use nom::{
    bytes::complete::tag, character::complete::alpha1, combinator::all_consuming, error::Error,
    multi::separated_list1, sequence::tuple, Finish, IResult,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn rotate_right(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    fn rotate_left(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    fn distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug)]
enum Move {
    Forward(u64),
    Direction(Direction, u64),
    Right(u64),
    Left(u64),
}

fn op(input: &str) -> IResult<&str, Move> {
    let (input, (op, arg)) = tuple((alpha1, number))(input)?;

    let op = match op {
        "N" => Move::Direction(Direction::North, arg),
        "E" => Move::Direction(Direction::East, arg),
        "S" => Move::Direction(Direction::South, arg),
        "W" => Move::Direction(Direction::West, arg),
        "R" => Move::Right(arg),
        "L" => Move::Left(arg),
        "F" => Move::Forward(arg),
        _ => panic!(),
    };

    Ok((input, op))
}

fn parse_program(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(tag("\n"), op)(input)
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Result<Vec<Move>, Error<String>> {
    match all_consuming(parse_program)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day12, part1)]
fn part1(data: &[Move]) -> usize {
    let mut position = Point::new(0, 0);
    let mut heading = Direction::East;

    for op in data {
        match op {
            Move::Forward(amount) => match heading {
                Direction::North => position.y += *amount as i64,
                Direction::East => position.x += *amount as i64,
                Direction::South => position.y -= *amount as i64,
                Direction::West => position.x -= *amount as i64,
            },
            Move::Direction(heading, amount) => match heading {
                Direction::North => position.y += *amount as i64,
                Direction::East => position.x += *amount as i64,
                Direction::South => position.y -= *amount as i64,
                Direction::West => position.x -= *amount as i64,
            },
            Move::Right(degrees) => {
                for _ in 0..degrees / 90 {
                    heading = heading.rotate_right();
                }
            }
            Move::Left(degrees) => {
                for _ in 0..degrees / 90 {
                    heading = heading.rotate_left();
                }
            }
        }
    }

    position.distance()
}

#[aoc(day12, part2)]
fn part2(data: &[Move]) -> usize {
    let mut position = Point::new(0, 0);
    let mut waypoint = Point::new(10, 1);

    for op in data {
        match op {
            Move::Forward(amount) => {
                position.x += waypoint.x * *amount as i64;
                position.y += waypoint.y * *amount as i64;
            }
            Move::Direction(heading, amount) => match heading {
                Direction::North => waypoint.y += *amount as i64,
                Direction::East => waypoint.x += *amount as i64,
                Direction::South => waypoint.y -= *amount as i64,
                Direction::West => waypoint.x -= *amount as i64,
            },
            Move::Right(degrees) => {
                for _ in 0..degrees / 90 {
                    waypoint = waypoint.rotate_right()
                }
            }
            Move::Left(degrees) => {
                for _ in 0..degrees / 90 {
                    waypoint = waypoint.rotate_left()
                }
            }
        }
    }

    position.distance()
}
