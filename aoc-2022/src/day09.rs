use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Move(Direction, usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }

    fn follow(&self, point: &Point) -> Option<Point> {
        let dx = point.x - self.x;
        let dy = point.y - self.y;
        if dx.abs() > 1 || dy.abs() > 1 {
            Some(Point {
                x: self.x + dx.clamp(-1, 1),
                y: self.y + dy.clamp(-1, 1),
            })
        } else {
            None
        }
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        separated_pair(
            alt((
                map(tag("L"), |_| Direction::Left),
                map(tag("R"), |_| Direction::Right),
                map(tag("U"), |_| Direction::Up),
                map(tag("D"), |_| Direction::Down),
            )),
            tag(" "),
            number,
        ),
        |(dir, step)| Move(dir, step),
    )(input)
}

fn parse_output(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(tag("\n"), parse_move)(input)
}

pub fn parse_input(input: &str) -> Vec<Move> {
    match all_consuming(terminated(parse_output, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn simulate_rope<const LEN: usize>(input: &[Move]) -> usize {
    let mut rope = [Point::default(); LEN];
    let mut visited = HashSet::new();

    for &Move(direction, steps) in input {
        'move_rope: for _ in 0..steps {
            rope[0] = rope[0].step(direction);

            for segment in 1..LEN {
                match rope[segment].follow(&rope[segment - 1]) {
                    Some(new_pos) => rope[segment] = new_pos,
                    None => continue 'move_rope,
                }
            }

            visited.insert(rope[LEN - 1]);
        }
    }

    visited.len()
}

pub fn part1(input: &[Move]) -> usize {
    simulate_rope::<2>(input)
}

pub fn part2(input: &[Move]) -> usize {
    simulate_rope::<10>(input)
}
