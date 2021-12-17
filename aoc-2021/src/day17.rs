use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::Error,
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use std::{iter, ops::RangeInclusive, str::FromStr};

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

pub fn x_coord(input: &str) -> IResult<&str, RangeInclusive<i32>> {
    map(
        preceded(tag("x="), separated_pair(number, tag(".."), number)),
        |(a, b): (i32, i32)| i32::min(a, b)..=i32::max(a, b),
    )(input)
}

pub fn y_coord(input: &str) -> IResult<&str, RangeInclusive<i32>> {
    map(
        preceded(tag("y="), separated_pair(number, tag(".."), number)),
        |(a, b): (i32, i32)| i32::min(a, b)..=i32::max(a, b),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (RangeInclusive<i32>, RangeInclusive<i32>)> {
    terminated(
        preceded(
            tag("target area: "),
            separated_pair(x_coord, tag(", "), y_coord),
        ),
        tag("\n"),
    )(input)
}

pub fn parse_input(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    match all_consuming(parse_rule)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Vector(i32, i32);

impl Vector {
    fn decay(&self) -> Self {
        let x = self.0 - self.0.signum();
        let y = self.1 - 1;
        Vector(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn advance(&mut self, vec: &Vector) -> Self {
        Self(self.0 + vec.0, self.1 + vec.1)
    }
}

fn simulate(mut vec: Vector, bounds: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> Option<i32> {
    let mut point = Point(0, 0);

    let path = iter::from_fn(move || {
        point = point.advance(&vec);
        vec = vec.decay();
        Some(point)
    })
    .take(1_000);

    let mut max_y = 0;
    for Point(x, y) in path {
        max_y = max_y.max(y);

        if bounds.0.contains(&x) && bounds.1.contains(&y) {
            return Some(max_y);
        } else if x > *bounds.0.end() || y < *bounds.1.start() {
            break;
        }
    }

    None
}

pub fn part1(target: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> i32 {
    let t = *target.0.end() + 1;

    (0..t)
        .flat_map(|x| (-t..t).map(move |y| Vector(x, y)))
        .flat_map(|vec| simulate(vec, target))
        .max()
        .unwrap()
}

pub fn part2(target: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> usize {
    let t = *target.0.end() + 1;

    (0..t)
        .flat_map(|x| (-t..t).map(move |y| Vector(x, y)))
        .flat_map(|vec| simulate(vec, target))
        .count()
}
