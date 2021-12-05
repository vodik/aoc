use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment(Point, Point);

impl Segment {
    fn is_orthogonal(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn points(&self) -> SegmentIter {
        if self.0.x == self.1.x {
            let x = self.0.x;
            let y0 = i32::min(self.0.y, self.1.y);
            let y1 = i32::max(self.0.y, self.1.y);

            SegmentIter::Vertical { y: (y0..=y1), x }
        } else if self.0.y == self.1.y {
            let y = self.0.y;
            let x0 = i32::min(self.0.y, self.1.y);
            let x1 = i32::max(self.0.y, self.1.y);

            SegmentIter::Horizontal { x: (x0..=x1), y }
        } else {
            let m = (self.1.y - self.0.y) / (self.1.x - self.0.x);
            let b = self.0.y - m * self.0.x;

            let x0 = i32::min(self.0.x, self.1.x);
            let x1 = i32::max(self.0.x, self.1.x);

            SegmentIter::Line { x: (x0..=x1), m, b }
        }
    }
}

enum SegmentIter {
    Horizontal {
        x: RangeInclusive<i32>,
        y: i32,
    },
    Vertical {
        x: i32,
        y: RangeInclusive<i32>,
    },
    Line {
        x: RangeInclusive<i32>,
        m: i32,
        b: i32,
    },
}

impl Iterator for SegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SegmentIter::Horizontal { x, y } => {
                let x = x.next()?;
                Some(Point::new(x, *y))
            }
            SegmentIter::Vertical { x, y } => {
                let y = y.next()?;
                Some(Point::new(*x, y))
            }
            SegmentIter::Line { x, m, b } => {
                let x = x.next()?;
                let y = x * *m + *b;
                Some(Point::new(x, y))
            }
        }
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

fn parse_line(input: &str) -> IResult<&str, Segment> {
    map(
        separated_pair(parse_point, tag(" -> "), parse_point),
        |(a, b)| Segment(a, b),
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<Segment>> {
    terminated(separated_list1(tag("\n"), parse_line), opt(tag("\n")))(input)
}

pub fn parse_input(input: &str) -> Vec<Segment> {
    match all_consuming(parse_file)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[Segment]) -> usize {
    let points = input
        .iter()
        .filter(|&segment| segment.is_orthogonal())
        .flat_map(|segment| segment.points());

    let mut map: HashMap<Point, usize> = HashMap::new();
    for point in points {
        let entry = map.entry(point).or_default();
        *entry += 1
    }

    map.into_values().filter(|&count| count > 1).count()
}

pub fn part2(input: &[Segment]) -> usize {
    let points = input.iter().flat_map(|segment| segment.points());

    let mut map: HashMap<Point, usize> = HashMap::new();
    for point in points {
        let entry = map.entry(point).or_default();
        *entry += 1
    }

    map.into_values().filter(|&count| count > 1).count()
}
