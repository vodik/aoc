use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Monkey {
    starting_items: Vec<u64>,
    operation: Expr,
    test: u64,
    true_dest: usize,
    false_dest: usize,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
pub struct Expr(Option<u64>, Op, Option<u64>);

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

fn parse_item_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(", "), number)(input)
}

fn parse_term(input: &str) -> IResult<&str, Option<u64>> {
    alt((map(tag("old"), |_| None), map(number, Some)))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            parse_term,
            preceded(
                tag(" "),
                alt((map(tag("*"), |_| Op::Mul), map(tag("+"), |_| Op::Add))),
            ),
            preceded(tag(" "), parse_term),
        )),
        |(lhs, op, rhs)| Expr(lhs, op, rhs),
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            tag("Monkey "),
            number::<u8>,
            tag(":"),
            preceded(tag("\n  Starting items: "), parse_item_list),
            preceded(tag("\n  Operation: new = "), parse_operation),
            preceded(tag("\n  Test: divisible by "), number),
            preceded(tag("\n    If true: throw to monkey "), number),
            preceded(tag("\n    If false: throw to monkey "), number),
        )),
        |(_, _, _, starting_items, operation, test, true_dest, false_dest)| Monkey {
            starting_items,
            operation,
            test,
            true_dest,
            false_dest,
        },
    )(input)
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), parse_monkey)(input)
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    match all_consuming(terminated(parse_monkeys, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

impl Expr {
    fn eval(&self, item: u64) -> u64 {
        let lhs = self.0.unwrap_or(item);
        let rhs = self.2.unwrap_or(item);
        match self.1 {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

impl Monkey {
    fn inspect_and_throw<const WORRY: u64>(&self, item: u64) -> (usize, u64) {
        let item = self.operation.eval(item) / WORRY;
        let dest = if item % self.test == 0 {
            self.true_dest
        } else {
            self.false_dest
        };
        (dest, item)
    }
}

pub fn part1(input: &[Monkey]) -> usize {
    let mut items: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(pos, monkey)| monkey.starting_items.iter().map(move |&item| (pos, item)))
        .collect();

    let mut moves = vec![0usize; input.len()];
    for _ in 0..20 {
        for pair in &mut items {
            let (mut pos, item) = *pair;

            let monkey = &input[pos];
            moves[pos] += 1;
            let (mut dest, mut item) = monkey.inspect_and_throw::<3>(item);
            while dest > pos {
                pos = dest;
                let monkey = &input[pos];
                moves[pos] += 1;
                (dest, item) = monkey.inspect_and_throw::<3>(item);
            }

            *pair = (dest, item);
        }
    }

    moves.sort_unstable();
    moves.iter().rev().take(2).product()
}

pub fn part2(input: &[Monkey]) -> usize {
    let gcd: u64 = input.iter().map(|monkey| monkey.test).product();
    let mut items: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(pos, monkey)| monkey.starting_items.iter().map(move |&item| (pos, item)))
        .collect();

    let mut moves = vec![0usize; input.len()];
    for _ in 0..10_000 {
        for pair in &mut items {
            let (mut pos, item) = *pair;

            let monkey = &input[pos];
            moves[pos] += 1;
            let (mut dest, mut item) = monkey.inspect_and_throw::<1>(item);
            while dest > pos {
                pos = dest;
                let monkey = &input[pos];
                moves[pos] += 1;
                (dest, item) = monkey.inspect_and_throw::<1>(item);
            }

            *pair = (dest, item % gcd);
        }
    }

    moves.sort_unstable();
    moves.iter().rev().take(2).product()
}
