use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, satisfy},
    combinator::{all_consuming, map, map_res, recognize},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug)]
pub enum CdOpt<'a> {
    Root,
    Up,
    Chdir(&'a str),
}

#[derive(Debug)]
pub enum Cmd<'a> {
    Cd(CdOpt<'a>),
    Ls,
}

#[derive(Debug)]
pub enum Output<'a> {
    Cmd(Cmd<'a>),
    File(u64),
    Directory(&'a str),
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_path(input: &str) -> IResult<&str, &str> {
    recognize(many1(satisfy(|c| c.is_ascii_graphic())))(input)
}

fn parse_output(input: &str) -> IResult<&str, Vec<Output>> {
    separated_list1(
        tag("\n"),
        alt((
            map(tag("$ cd /"), |_| Output::Cmd(Cmd::Cd(CdOpt::Root))),
            map(tag("$ cd .."), |_| Output::Cmd(Cmd::Cd(CdOpt::Up))),
            map(preceded(tag("$ cd "), parse_path), |path| {
                Output::Cmd(Cmd::Cd(CdOpt::Chdir(path)))
            }),
            map(tag("$ ls"), |_| Output::Cmd(Cmd::Ls)),
            map(terminated(tag("dir "), parse_path), Output::Directory),
            map(tuple((number, tag(" "), parse_path)), |(size, _, _)| {
                Output::File(size)
            }),
        )),
    )(input)
}

pub fn parse_input(input: &str) -> Vec<u64> {
    match all_consuming(terminated(parse_output, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .map(|ops| filesystem_from_replay(&ops))
    .unwrap()
}

fn filesystem_from_replay(input: &[Output]) -> Vec<u64> {
    let mut folders = Vec::with_capacity(200);
    let mut stack = Vec::with_capacity(20);

    stack.push(0);
    for op in &input[1..] {
        match op {
            Output::Cmd(Cmd::Cd(CdOpt::Root)) => todo!(),
            Output::Cmd(Cmd::Cd(CdOpt::Up)) => {
                let dir_size = stack.pop().unwrap();
                folders.push(dir_size);
                *stack.last_mut().unwrap() += dir_size;
            }
            Output::Cmd(Cmd::Cd(CdOpt::Chdir(_))) => {
                stack.push(0);
            }
            Output::File(size) => {
                *stack.last_mut().unwrap() += *size;
            }
            _ => {}
        }
    }

    folders.extend(stack.drain(..).rev().scan(0, |acc, dir_size| {
        let cur_size = dir_size + *acc;
        *acc += dir_size;
        Some(cur_size)
    }));

    folders
}

pub fn part1(input: &[u64]) -> u64 {
    input.iter().rev().filter(|&&size| size <= 100_000).sum()
}

pub fn part2(input: &[u64]) -> u64 {
    let mut sizes = input.iter().rev();

    let used = sizes.next().unwrap();
    let free = 70_000_000 - used;
    let needed = 30_000_000 - free;

    sizes
        .filter(|&&size| size >= needed)
        .min()
        .copied()
        .unwrap()
}
