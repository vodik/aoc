use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::{collections::HashSet, fmt::Write, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    Right(usize),
    Up(usize),
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(number, tag(","), number), |(x, y)| {
        Point::new(x, y)
    })(input)
}

fn parse_fold(input: &str) -> IResult<&str, Fold> {
    preceded(
        tag("fold along "),
        map(
            separated_pair(anychar, tag("="), number),
            |(axis, pos)| match axis {
                'x' => Fold::Right(pos),
                'y' => Fold::Up(pos),
                _ => unreachable!(),
            },
        ),
    )(input)
}

fn parse_points(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag("\n"), parse_point)(input)
}

fn parse_folds(input: &str) -> IResult<&str, Vec<Fold>> {
    separated_list1(tag("\n"), parse_fold)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<Point>, Vec<Fold>)> {
    terminated(
        separated_pair(parse_points, tag("\n\n"), parse_folds),
        opt(tag("\n")),
    )(input)
}

pub fn parse_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
    match all_consuming(parse_file)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn fold_board(input: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    input
        .iter()
        .copied()
        .map(|Point { x, y }| match *fold {
            Fold::Right(pos) => {
                let x = if x > pos as i32 {
                    x - (x - pos as i32) * 2
                } else {
                    x
                };
                Point::new(x, y)
            }
            Fold::Up(pos) => {
                let y = if y > pos as i32 {
                    y - (y - pos as i32) * 2
                } else {
                    y
                };
                Point::new(x, y)
            }
        })
        .collect()
}

pub fn part1((points, folds): &(Vec<Point>, Vec<Fold>)) -> usize {
    let board: HashSet<Point> = points.iter().copied().collect();
    fold_board(&board, &folds[0]).len()
}

fn dump(board: &HashSet<Point>) -> String {
    let max_x = board.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = board.iter().map(|p| p.y).max().unwrap() + 1;
    let mut output = String::with_capacity(max_x as usize * max_y as usize);

    for y in 0..max_y {
        for x in 0..max_x {
            let p = Point { x, y };
            if board.contains(&p) {
                write!(&mut output, "#").unwrap();
            } else {
                write!(&mut output, ".").unwrap();
            }
        }
        writeln!(&mut output).unwrap();
    }
    output
}

pub fn part2((points, folds): &(Vec<Point>, Vec<Fold>)) -> usize {
    let mut board: HashSet<Point> = points.iter().copied().collect();
    for fold in folds {
        board = fold_board(&board, fold);
    }

    // FIXME: simplistic runner can't handle strings yet
    print!("{}", dump(&board));
    0
}
