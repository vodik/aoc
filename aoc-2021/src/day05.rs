use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use std::{ops::Range, str::FromStr};

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Segment(Point, Point);

impl Segment {
    fn new(a: Point, b: Point) -> Self {
        Segment(a, b)
    }

    fn is_orthogonal(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn interpolate(&self) -> SegmentIter {
        if self.0.x == self.1.x {
            let x = self.0.x;
            let y0 = i32::min(self.0.y, self.1.y);
            let y1 = i32::max(self.0.y, self.1.y) + 1;

            SegmentIter::Vertical {
                y_iter: (y0..y1),
                x,
            }
        } else if self.0.y == self.1.y {
            let x0 = i32::min(self.0.x, self.1.x);
            let x1 = i32::max(self.0.x, self.1.x) + 1;
            let y = self.0.y;

            SegmentIter::Horizontal {
                x_iter: (x0..x1),
                y,
            }
        } else {
            let x0 = i32::min(self.0.x, self.1.x);
            let x1 = i32::max(self.0.x, self.1.x) + 1;
            let m = (self.1.y - self.0.y) / (self.1.x - self.0.x);
            let b = self.0.y - m * self.0.x;

            SegmentIter::Sloped {
                x_iter: (x0..x1),
                m,
                b,
            }
        }
    }
}

enum SegmentIter {
    Vertical {
        x: i32,
        y_iter: Range<i32>,
    },
    Horizontal {
        x_iter: Range<i32>,
        y: i32,
    },
    Sloped {
        x_iter: Range<i32>,
        m: i32,
        b: i32,
    },
}

impl Iterator for SegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            SegmentIter::Vertical { x, ref mut y_iter } => {
                let y = y_iter.next()?;
                Some(Point::new(x, y))
            }
            SegmentIter::Horizontal { ref mut x_iter, y } => {
                let x = x_iter.next()?;
                Some(Point::new(x, y))
            }
            SegmentIter::Sloped {
                ref mut x_iter,
                m,
                b,
            } => {
                let x = x_iter.next()?;
                let y = m * x + b;
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
        |(a, b)| Segment::new(a, b),
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

fn count_points_of_interest(points: impl Iterator<Item = Point>) -> usize {
    let mut map = [0u8; 1000 * 1000];
    let mut points_of_interest = 0;

    for Point { x, y } in points {
        let index = usize::try_from(x).unwrap() * 1000 + usize::try_from(y).unwrap();
        let count = &mut map[index];

        *count = count.saturating_add(1);
        if *count == 2 {
            points_of_interest += 1;
        }
    }

    points_of_interest
}

pub fn part1(input: &[Segment]) -> usize {
    count_points_of_interest(
        input
            .iter()
            .filter(|segment| segment.is_orthogonal())
            .flat_map(|segment| segment.interpolate()),
    )
}

pub fn part2(input: &[Segment]) -> usize {
    count_points_of_interest(input.iter().flat_map(|segment| segment.interpolate()))
}
