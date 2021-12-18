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
pub struct SnailNumber(Vec<(u8, usize)>);

fn push(n: &Expr, depth: usize, sn: &mut SnailNumber) {
    match n {
        Expr::Pair(a, b) => {
            push(a, depth + 1, sn);
            push(b, depth + 1, sn);
        }
        Expr::Value(v) => {
            sn.0.push((*v, depth));
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
        self.0.extend_from_slice(&other.0);
        self.0.iter_mut().for_each(|(_, c)| *c += 1);
    }

    fn explode(&mut self, pos: usize) -> bool {
        if self.0[pos].1 != 5 {
            return false;
        }

        if pos > 0 {
            self.0[pos - 1].0 += self.0[pos].0;
        }
        if pos + 2 < self.0.len() {
            self.0[pos + 2].0 += self.0[pos + 1].0;
        }

        self.0.remove(pos);
        self.0[pos].0  = 0;
        self.0[pos].1 -= 1;

        true
    }

    fn explode_scan(&mut self, hint: usize) -> Option<usize> {
        if let Some(pos) = self.0[hint..].iter().position(|&(_, d)| d == 5) {
            self.explode(pos + hint).then(|| pos + hint)
        } else {
            None
        }
    }

    fn split(&mut self, pos: usize) -> bool {
        let cell = &mut self.0[pos];
        if cell.0 < 10 {
            return false;
        }

        let left = cell.0 / 2;
        let right = cell.0 - left;
        let depth = cell.1 + 1;

        *cell = (left, depth);
        self.0.insert(pos + 1, (right, depth));

        true
    }

    fn split_scan(&mut self, hint: usize) -> Option<usize> {
        if let Some(pos) = self.0[hint..].iter().position(|&(v, _)| v >= 10) {
            self.split(pos + hint).then(|| pos + hint)
        } else {
            None
        }
    }

    fn reduce(&mut self) {
        let mut hint = 0;
        loop {
            let mut reset = false;
            while let Some(newhint) = self.explode_scan(hint) {
                reset = true;
                hint = newhint;
            }

            if reset {
                hint = 0;
            }

            if let Some(newhint) = self.split_scan(hint) {
                hint = newhint
            } else {
                break;
            }
        }
    }

    fn magnitude(&self) -> u64 {
        let mut numbers: Vec<_> = self.0.iter().copied().map(|(v, d)| (v.into(), d)).collect();

        for depth in 0..4 {
            let depth = 4 - depth;

            let mut left = 0;
            while left < numbers.len() {
                if numbers[left].1 != depth {
                    left += 1;
                } else {
                    let mut right = left + 1;
                    while numbers[right].1 == 0 {
                        right += 1;
                    }

                    numbers[left].0 = numbers[left].0 * 3 + numbers[right].0 * 2;
                    numbers[left].1 -= 1;
                    numbers[right].1 = 0;

                    left = right + 1;
                }
            }
        }

        numbers[0].0
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
