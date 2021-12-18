use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    Finish, IResult,
};
use std::str::FromStr;

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

#[derive(Debug)]
pub enum Expr {
    Pair(Box<Expr>, Box<Expr>),
    Value(u8),
}

fn pair(input: &str) -> IResult<&str, Expr> {
    delimited(
        tag("["),
        map(
            separated_pair(snail_number, tag(","), snail_number),
            |(left, right)| Expr::Pair(Box::new(left), Box::new(right)),
        ),
        tag("]"),
    )(input)
}

fn snail_number(input: &str) -> IResult<&str, Expr> {
    alt((map(number, Expr::Value), pair))(input)
}

fn all_numbers(input: &str) -> IResult<&str, Vec<SnailNumber>> {
    separated_list1(tag("\n"), map(snail_number, |sn| SnailNumber::parse(&sn)))(input)
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
    digits: Vec<u8>,
    depths: Vec<usize>,
}

fn push(n: &Expr, depth: usize, sn: &mut SnailNumber) {
    match n {
        Expr::Pair(a, b) => {
            push(a, depth + 1, sn);
            push(b, depth + 1, sn);
        }
        Expr::Value(v) => {
            sn.digits.push(*v);
            sn.depths.push(depth);
        }
    }
}

impl SnailNumber {
    fn parse(n: &Expr) -> Self {
        let mut sn = SnailNumber::default();
        push(n, 0, &mut sn);
        sn
    }

    fn add(&mut self, other: &SnailNumber) {
        self.digits.extend_from_slice(&other.digits);
        self.depths.iter_mut().for_each(|c| *c += 1);
        self.depths.extend(other.depths.iter().map(|c| *c + 1));
    }

    fn explode(&mut self) -> bool {
        if let Some(pos) = self.depths.iter().position(|&d| d == 5) {
            if pos > 0 {
                self.digits[pos - 1] += self.digits[pos];
            }
            if pos + 2 < self.digits.len() {
                self.digits[pos + 2] += self.digits[pos + 1];
            }

            self.digits.remove(pos);
            self.digits[pos] = 0;

            self.depths.remove(pos);
            self.depths[pos] -= 1;

            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        if let Some(pos) = self.digits.iter().position(|&v| v >= 10) {
            let value = self.digits[pos];
            let left = value / 2;
            let right = value - left;

            self.digits[pos] = left;
            self.depths[pos] += 1;

            self.digits.insert(pos + 1, right);
            self.depths.insert(pos + 1, self.depths[pos]);

            true
        } else {
            false
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn magnitude(&self) -> u64 {
        let mut numbers: Vec<_> = self.digits.iter().copied().map(Into::into).collect();
        let mut depths = self.depths.clone();

        for depth in 0..4 {
            let depth = 4 - depth;

            let mut left = 0;
            while left < depths.len() {
                if depths[left] != depth {
                    left += 1;
                } else {
                    let mut right = left + 1;
                    while depths[right] == 0 {
                        right += 1;
                    }

                    numbers[left] = numbers[left] * 3 + numbers[right] * 2;
                    depths[left] -= 1;
                    depths[right] = 0;

                    left = right + 1;
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
