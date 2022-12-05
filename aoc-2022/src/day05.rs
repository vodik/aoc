use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, satisfy},
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};
use std::str::FromStr;

pub struct Op {
    amount: usize,
    src: usize,
    dest: usize,
}

enum Token {
    Empty,
    Crate(u8),
    Label,
}

impl Op {
    fn new(amount: usize, src: usize, dest: usize) -> Self {
        Self { amount, src, dest }
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_crate(input: &str) -> IResult<&str, u8> {
    delimited(
        tag("["),
        map(satisfy(|c| c.is_ascii_alphabetic()), |c| c as u8),
        tag("]"),
    )(input)
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    delimited(tag(" "), digit1, opt(tag(" ")))(input)
}

fn parse_stack_desc(input: &str) -> IResult<&str, Vec<Token>> {
    separated_list1(
        tag(" "),
        alt((
            map(tag("   "), |_| Token::Empty),
            map(parse_crate, Token::Crate),
            map(parse_label, |_| Token::Label),
        )),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Op> {
    map(
        tuple((
            preceded(tag("move "), number),
            preceded(tag(" from "), number),
            preceded(tag(" to "), number),
        )),
        |(amount, src, dest): (usize, usize, usize)| Op::new(amount, src - 1, dest - 1),
    )(input)
}

fn build_stacks(lines: Vec<Vec<Token>>) -> Vec<Vec<u8>> {
    let width = lines.last().unwrap().len();
    let mut state = vec![vec![]; width];
    for line in lines.into_iter().rev().skip(1) {
        for (cell, row) in line.iter().zip(state.iter_mut()) {
            if let &Token::Crate(c) = cell {
                row.push(c);
            }
        }
    }
    state
}

fn parse_stacks_and_procedure(input: &str) -> IResult<&str, (Vec<Vec<u8>>, Vec<Op>)> {
    tuple((
        map(separated_list1(tag("\n"), parse_stack_desc), build_stacks),
        preceded(tag("\n\n"), separated_list1(tag("\n"), parse_operation)),
    ))(input)
}

pub fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Op>) {
    match all_consuming(terminated(parse_stacks_and_procedure, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

trait CrateMover {
    fn lift(&mut self, src: &mut Vec<u8>, amount: usize);

    fn place(&mut self, dest: &mut Vec<u8>);
}

struct CrateMover9000(Vec<u8>);

impl CrateMover9000 {
    const fn new() -> Self {
        Self(Vec::new())
    }
}

impl CrateMover for CrateMover9000 {
    fn lift(&mut self, src: &mut Vec<u8>, amount: usize) {
        for _ in 0..amount {
            if let Some(c) = src.pop() {
                self.0.push(c);
            }
        }
    }

    fn place(&mut self, dest: &mut Vec<u8>) {
        dest.append(&mut self.0)
    }
}

struct CrateMover9001(Vec<u8>);

impl CrateMover9001 {
    const fn new() -> Self {
        Self(Vec::new())
    }
}

impl CrateMover for CrateMover9001 {
    fn lift(&mut self, src: &mut Vec<u8>, amount: usize) {
        for _ in 0..amount {
            if let Some(c) = src.pop() {
                self.0.push(c);
            }
        }
    }

    fn place(&mut self, dest: &mut Vec<u8>) {
        dest.extend(self.0.drain(..).rev())
    }
}

fn simulate(state: &mut [Vec<u8>], moves: &[Op], mut mover: impl CrateMover) {
    for &Op { amount, src, dest } in moves {
        let src = &mut state[src];
        mover.lift(src, amount);

        let dest = &mut state[dest];
        mover.place(dest);
    }
}

fn top_of_stacks(state: &[Vec<u8>]) -> String {
    let output: Vec<_> = state
        .iter()
        .map(|stack| stack.last().copied().unwrap_or(b' '))
        .collect();
    String::from_utf8(output).unwrap()
}

pub fn part1((state, moves): &(Vec<Vec<u8>>, Vec<Op>)) -> String {
    let mut state = state.clone();
    simulate(&mut state, moves, CrateMover9000::new());
    top_of_stacks(&state)
}

pub fn part2((state, moves): &(Vec<Vec<u8>>, Vec<Op>)) -> String {
    let mut state = state.clone();
    simulate(&mut state, moves, CrateMover9001::new());
    top_of_stacks(&state)
}
