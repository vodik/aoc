use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    combinator::{all_consuming, map_res, recognize},
    error::Error,
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct Program {
    prog: Vec<(Op, i32)>,
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

impl Program {
    fn new(prog: Vec<(Op, i32)>) -> Self {
        Self {
            prog,
            pc: 0,
            acc: 0,
            trace: HashSet::new(),
        }
    }

    fn step(&mut self) -> State {
        if let Some((op, arg)) = self.prog.get(self.pc) {
            if self.trace.contains(&self.pc) {
                return State::Blocked;
            }
            self.trace.insert(self.pc);

            match op {
                Op::Nop => self.pc += 1,
                Op::Acc => {
                    self.acc += arg;
                    self.pc += 1
                }
                Op::Jmp => self.pc = (self.pc as i32 + *arg).try_into().unwrap(),
            };

            State::Running
        } else {
            State::Halted
        }
    }
}

fn op(input: &str) -> IResult<&str, (Op, i32)> {
    let (input, op) = alpha1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = map_res(recognize(tuple((one_of("+-"), digit1))), str::parse)(input)?;

    let op = match op {
        "nop" => Op::Nop,
        "acc" => Op::Acc,
        "jmp" => Op::Jmp,
        _ => panic!(),
    };

    Ok((input, (op, arg)))
}

fn parse_program(input: &str) -> IResult<&str, Vec<(Op, i32)>> {
    separated_list1(tag("\n"), op)(input)
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Result<Vec<(Op, i32)>, Error<String>> {
    match all_consuming(parse_program)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day8, part1)]
fn part1(data: &[(Op, i32)]) -> Option<i32> {
    let mut prog = Program::new(data.to_vec());

    loop {
        match prog.step() {
            State::Blocked => break Some(prog.acc),
            State::Halted => break None,
            _ => {}
        }
    }
}

#[aoc(day8, part2)]
fn part2(data: &[(Op, i32)]) -> Option<i32> {
    data.iter()
        .enumerate()
        .filter_map(|(idx, (op, arg))| {
            let replacement = match op {
                Op::Nop => Some(Op::Jmp),
                Op::Jmp => Some(Op::Nop),
                _ => None,
            };

            replacement.map(|op| {
                let mut new_prog = data.to_vec();
                new_prog[idx] = (op, *arg);
                new_prog
            })
        })
        .find_map(|data| {
            let mut prog = Program::new(data);

            loop {
                match prog.step() {
                    State::Blocked => break None,
                    State::Halted => break Some(prog.acc),
                    _ => {}
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_description() {
        assert_eq!(op("nop +0"), Ok(("", ("nop".to_string(), 0))));
    }
}
