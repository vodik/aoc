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

pub fn part1(input: &[Op]) -> i64 {
    let mut addx_running = None;
    let mut pc = 0;
    let mut strength = 1;
    let mut sum = 0;

    for cycle in 1..=220 {
        let mut next_strength = strength;

        if let Some(delay) = addx_running.take() {
            next_strength = delay;
        } else {
            match &input[pc] {
                Op::Noop => {}
                Op::Addx(value) => {
                    addx_running = Some(strength + *value);
                }
            }
            pc += 1;
        }

        if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
            sum += cycle as i64 * strength;
        }

        strength = next_strength;
    }

    sum
}

pub fn part2(input: &[Op]) -> u64 {
    let mut addx_running = None;
    let mut pc = 0;
    let mut strength = 1;

    let mut screen = vec![b' '; 40 * 6];

    for cycle in 1..=240 {
        let mut next_strength = strength;

        if let Some(delay) = addx_running.take() {
            next_strength = delay;
        } else {
            match &input[pc] {
                Op::Noop => {}
                Op::Addx(value) => {
                    addx_running = Some(strength + *value);
                }
            }
            pc += 1;
        }

        let pixel = cycle - 1;
        if [strength - 1, strength, strength + 1].contains(&(pixel % 40)) {
            screen[pixel as usize] = b'#';
        }

        strength = next_strength;
    }

    for row in screen.chunks(40) {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
    0
}
