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

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Expr,
    test: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Term {
    Old,
    Literal(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
pub struct Expr(Term, Op, Term);

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

fn parse_item_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(", "), number)(input)
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((map(tag("old"), |_| Term::Old), map(number, Term::Literal)))(input)
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
        |(_, _, _, items, operation, test, if_true, if_false)| Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
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
        let lhs = match self.0 {
            Term::Old => item,
            Term::Literal(literal) => literal,
        };

        let rhs = match self.2 {
            Term::Old => item,
            Term::Literal(literal) => literal,
        };

        match self.1 {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

impl Monkey {
    fn inspect_and_throw<const WORRY: u64>(&mut self) -> impl Iterator<Item = (usize, u64)> + '_ {
        self.items.drain(..).map(|item| {
            let mut item = self.operation.eval(item);
            item /= WORRY;
            if item % self.test == 0 {
                (self.if_true, item)
            } else {
                (self.if_false, item)
            }
        })
    }
}

pub fn part1(input: &[Monkey]) -> usize {
    let mut monkeys = input.to_vec();
    let mut moves = vec![0usize; monkeys.len()];

    let mut buf = Vec::with_capacity(200);
    for _ in 0..20 {
        for pos in 0..monkeys.len() {
            moves[pos] += monkeys[pos].items.len();
            buf.extend(monkeys[pos].inspect_and_throw::<3>());
            for (target, item) in buf.drain(..) {
                monkeys[target].items.push(item);
            }
        }
    }

    moves.sort_unstable();
    moves.iter().rev().take(2).product()
}

pub fn part2(input: &[Monkey]) -> usize {
    let mut monkeys = input.to_vec();
    let mut moves = vec![0usize; monkeys.len()];
    let gcd: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    let mut buf = Vec::with_capacity(200);
    for _ in 0..10_000 {
        for pos in 0..monkeys.len() {
            moves[pos] += monkeys[pos].items.len();
            buf.extend(monkeys[pos].inspect_and_throw::<1>());
            for (target, item) in buf.drain(..) {
                monkeys[target].items.push(item % gcd);
            }
        }
    }

    moves.sort_unstable();
    moves.iter().rev().take(2).product()
}
