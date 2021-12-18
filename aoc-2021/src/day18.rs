use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::str::FromStr;

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

#[derive(Debug)]
pub enum Number {
    Pair(Box<Number>, Box<Number>, usize),
    Value(u8, usize),
}

fn pair(depth: usize) -> impl Fn(&str) -> IResult<&str, Number> {
    move |input: &str| {
        terminated(
            preceded(
                tag("["),
                map(
                    separated_pair(snail_number(depth + 1), tag(","), snail_number(depth + 1)),
                    |(a, b)| Number::Pair(Box::new(a), Box::new(b), depth),
                ),
            ),
            tag("]"),
        )(input)
    }
}

fn snail_number(depth: usize) -> impl Fn(&str) -> IResult<&str, Number> {
    move |input: &str| alt((map(number, |n| Number::Value(n, depth)), pair(depth)))(input)
}

fn all_numbers(input: &str) -> IResult<&str, Vec<SnailNumber>> {
    separated_list1(
        tag("\n"),
        map(snail_number(0), |sn| SnailNumber::parse(&sn)),
    )(input)
}

pub fn parse_input(input: &str) -> Vec<SnailNumber> {
    match all_consuming(terminated(all_numbers, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

#[derive(Debug, Default, Clone)]
pub struct SnailNumber {
    numbers: Vec<u8>,
    depth: Vec<usize>,
}

fn push(n: &Number, depth: usize, sn: &mut SnailNumber) {
    match n {
        Number::Pair(a, b, depth) => {
            push(a, *depth, sn);
            push(b, *depth, sn);
        }
        Number::Value(v, depth) => {
            sn.numbers.push(*v);
            sn.depth.push(*depth);
        }
    }
}

impl SnailNumber {
    fn parse(n: &Number) -> Self {
        let mut sn = SnailNumber::default();
        push(n, 0, &mut sn);
        sn
    }

    fn add(&mut self, other: &SnailNumber) {
        self.numbers.extend_from_slice(&other.numbers);
        self.depth.iter_mut().for_each(|c| *c += 1);
        self.depth.extend(other.depth.iter().map(|c| *c + 1));
    }

    fn explode(&mut self) -> bool {
        if let Some(pos) = self.depth.iter().position(|&d| d == 5) {
            if pos > 0 {
                self.numbers[pos - 1] += self.numbers[pos];
            }
            if pos + 2 < self.numbers.len() {
                self.numbers[pos + 2] += self.numbers[pos + 1];
            }

            self.numbers.remove(pos);
            self.depth.remove(pos);

            self.numbers[pos] = 0;
            self.depth[pos] -= 1;
            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        if let Some(pos) = self.numbers.iter().position(|&v| v >= 10) {
            let value = self.numbers[pos];
            let left = value / 2;
            let right = value - left;

            self.depth[pos] += 1;
            self.depth.insert(pos + 1, self.depth[pos]);
            self.numbers[pos] = left;
            self.numbers.insert(pos + 1, right);
            true
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn magnitude(&self) -> u64 {
        let mut numbers: Vec<u64> = self.numbers.iter().copied().map(Into::into).collect();
        let mut depths = self.depth.clone();

        for depth in 0..4 {
            let depth = 4 - depth;

            for left in 0..depths.len() {
                if depths[left] == depth {
                    let mut right = left + 1;
                    while depths[right] == 0 {
                        right += 1;
                    }

                    numbers[left] = numbers[left] * 3 + numbers[right] * 2;
                    depths[left] -= 1;
                    depths[right] = 0;
                }
            }
        }

        numbers[0]
    }
}

pub fn part1(input: &[SnailNumber]) -> u64 {
    let first = input[0].clone();

    input[1..]
        .iter()
        .fold(first, |mut acc, sn| {
            acc.add(sn);
            acc.reduce();
            acc
        })
        .magnitude()
}

pub fn part2(input: &[SnailNumber]) -> u64 {
    let mut max = 0;
    for pos in 0..input.len() {
        for other in 0..input.len() {
            if other == pos {
                continue;
            }

            let mut sn = input[pos].clone();
            sn.add(&input[other]);
            sn.reduce();
            max = max.max(sn.magnitude());
        }
    }

    max
}
