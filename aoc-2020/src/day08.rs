use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    combinator::{all_consuming, map, map_res, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};
use std::{collections::HashSet, convert::TryInto};

#[derive(Debug, Clone, Copy)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
struct Machine {
    program: Vec<Op>,
    pc: usize,
    acc: i32,
    trace: HashSet<usize>,
}

#[derive(Debug)]
enum State {
    Running,
    Blocked,
    Halted,
}

impl Machine {
    fn new(program: Vec<Op>) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
            trace: HashSet::new(),
        }
    }

    fn step(&mut self) -> State {
        if let Some(op) = self.program.get(self.pc) {
            if !self.trace.insert(self.pc) {
                return State::Blocked;
            }

            match op {
                Op::Nop(_) => self.pc += 1,
                Op::Acc(arg) => {
                    self.acc += arg;
                    self.pc += 1
                }
                Op::Jmp(arg) => self.pc = (self.pc as i32 + *arg).try_into().unwrap(),
            };

            State::Running
        } else {
            State::Halted
        }
    }
}

fn op(input: &str) -> IResult<&str, Op> {
    map(
        separated_pair(
            alpha1,
            tag(" "),
            map_res(recognize(tuple((one_of("+-"), digit1))), str::parse),
        ),
        |(op, arg)| match op {
            "nop" => Op::Nop(arg),
            "acc" => Op::Acc(arg),
            "jmp" => Op::Jmp(arg),
            _ => panic!(),
        },
    )(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(tag("\n"), op)(input)
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Result<Vec<Op>, Error<String>> {
    match all_consuming(parse_program)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day8, part1)]
fn part1(data: &[Op]) -> Option<i32> {
    let mut machine = Machine::new(data.to_vec());

    loop {
        match machine.step() {
            State::Blocked => break Some(machine.acc),
            State::Halted => break None,
            _ => {}
        }
    }
}

#[aoc(day8, part2)]
fn part2(data: &[Op]) -> Option<i32> {
    data.iter()
        .enumerate()
        .filter_map(|(idx, op)| {
            let replacement = match op {
                Op::Nop(arg) => Some(Op::Jmp(*arg)),
                Op::Jmp(arg) => Some(Op::Nop(*arg)),
                _ => None,
            };

            replacement.map(|op| {
                let mut new_program = data.to_vec();
                new_program[idx] = op;
                new_program
            })
        })
        .find_map(|data| {
            let mut machine = Machine::new(data);

            loop {
                match machine.step() {
                    State::Blocked => break None,
                    State::Halted => break Some(machine.acc),
                    _ => {}
                }
            }
        })
}
