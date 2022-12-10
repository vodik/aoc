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
pub enum Op {
    Noop,
    Addx(i64),
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

fn parse_ops(input: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(
        tag("\n"),
        alt((
            map(tag("noop"), |_| Op::Noop),
            map(preceded(tag("addx "), number), Op::Addx),
        )),
    )(input)
}

pub fn parse_input(input: &str) -> Vec<Op> {
    match all_consuming(terminated(parse_ops, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

struct Cpu<'a> {
    memory: &'a [Op],
    addx_running: Option<i64>,
    x: i64,
    pc: usize,
}

impl<'a> Cpu<'a> {
    fn new(memory: &'a [Op]) -> Self {
        Self {
            memory,
            addx_running: None,
            x: 1,
            pc: 0,
        }
    }

    fn step(&mut self) -> i64 {
        if let Some(delay) = self.addx_running.take() {
            let x = self.x;
            self.x = delay;
            x
        } else {
            match &self.memory[self.pc] {
                Op::Noop => {}
                Op::Addx(value) => {
                    self.addx_running = Some(self.x + *value);
                }
            }
            self.pc += 1;
            self.x
        }
    }
}

pub fn part1(input: &[Op]) -> i64 {
    let mut cpu = Cpu::new(input);
    let mut sum = 0;

    for cycle in 1..=220 {
        let x = cpu.step();
        if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
            sum += cycle as i64 * x;
        }
    }

    sum
}

pub fn part2(input: &[Op]) {
    let mut cpu = Cpu::new(input);
    let mut screen = [b' '; 40 * 6];

    for cycle in 1..=240 {
        let pixel = cycle - 1;
        let x = cpu.step();
        if (x - 1..x + 2).contains(&(pixel % 40)) {
            screen[pixel as usize] = b'#';
        }
    }

    for row in screen.chunks(40) {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}
