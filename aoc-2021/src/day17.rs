use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::Error,
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use std::{ops::Range, str::FromStr};

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

pub fn x_coord(input: &str) -> IResult<&str, Range<i32>> {
    map(
        preceded(tag("x="), separated_pair(number, tag(".."), number)),
        |(a, b): (i32, i32)| i32::min(a, b)..i32::max(a, b) + 1,
    )(input)
}

pub fn y_coord(input: &str) -> IResult<&str, Range<i32>> {
    map(
        preceded(tag("y="), separated_pair(number, tag(".."), number)),
        |(a, b): (i32, i32)| i32::min(a, b)..i32::max(a, b) + 1,
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (Range<i32>, Range<i32>)> {
    terminated(
        preceded(
            tag("target area: "),
            separated_pair(x_coord, tag(", "), y_coord),
        ),
        tag("\n"),
    )(input)
}

pub fn parse_input(input: &str) -> (Range<i32>, Range<i32>) {
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

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

fn simulate(vec: Vector, bounds: &(Range<i32>, Range<i32>)) -> Option<i32> {
    let trajectory = (1..vec.0 + 1)
        .map(|n| {
            let t = n * (n - 1) / 2;
            Point(vec.0 * n - t, vec.1 * n - t)
        })
        .take_while(|&Point(x, _)| x < bounds.0.end);

    let mut max_y = 0;
    for Point(x, y) in trajectory {
        max_y = max_y.max(y);
        if bounds.0.contains(&x) && bounds.1.contains(&y) {
            return Some(max_y);
        }
    }

    if !vec.1.is_positive() {
        return None;
    }

    let max_x = vec.0 * (vec.0 + 1) / 2;
    if !bounds.0.contains(&max_x) {
        return None;
    }

    let max_y = vec.1 * (vec.1 + 1) / 2;
    (1..)
        .map(|n| {
            let t = n * (n - 1) / 2;
            max_y - t
        })
        .take_while(|&y| y >= bounds.1.start)
        .any(|y| bounds.1.contains(&y))
        .then(|| max_y)
}

pub fn part1(target: &(Range<i32>, Range<i32>)) -> i32 {
    let t = target.0.end;

    (0..t)
        .flat_map(|x| (-t..t).map(move |y| Vector(x, y)))
        .flat_map(|vec| simulate(vec, target))
        .max()
        .unwrap()
}

pub fn part2(target: &(Range<i32>, Range<i32>)) -> usize {
    let t = target.0.end;

    (0..t)
        .flat_map(|x| (-t..t).map(move |y| Vector(x, y)))
        .flat_map(|vec| simulate(vec, target))
        .count()
}
