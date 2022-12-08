use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, satisfy},
    combinator::{all_consuming, map, map_res, recognize},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};
use std::{
    collections::{btree_map::Entry, BTreeMap},
    str::FromStr,
};

#[derive(Debug, Default)]
pub struct Node {
    size: u64,
    children: BTreeMap<String, usize>,
}

#[derive(Debug)]
pub struct Filesystem {
    nodes: Vec<Node>,
}

impl Filesystem {
    const ROOT: usize = 0;

    fn new() -> Self {
        Filesystem {
            nodes: vec![Node::default()],
        }
    }

    fn open_at(&mut self, dirfd: usize, path: &str) -> usize {
        let next_fd = self.nodes.len();
        let dir = &mut self.nodes[dirfd];
        match dir.children.entry(path.into()) {
            Entry::Vacant(entry) => {
                entry.insert(next_fd);
                self.nodes.push(Default::default());
                next_fd
            }
            Entry::Occupied(entry) => *entry.get(),
        }
    }

    fn get(&self, dirfd: usize) -> Option<&Node> {
        self.nodes.get(dirfd)
    }

    fn get_mut(&mut self, dirfd: usize) -> Option<&mut Node> {
        self.nodes.get_mut(dirfd)
    }
}

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
    recognize(many1(satisfy(|c| c.is_ascii_alphanumeric() || c == '.')))(input)
}

fn parse_output(input: &str) -> IResult<&str, Vec<Output>> {
    separated_list1(
        tag("\n"),
        alt((
            map(tag("$ cd /"), |_| Output::Cmd(Cmd::Cd(CdOpt::Root))),
            map(tag("$ cd .."), |_| Output::Cmd(Cmd::Cd(CdOpt::Up))),
            map(preceded(tag("$ cd "), alpha1), |path: &str| {
                Output::Cmd(Cmd::Cd(CdOpt::Chdir(path)))
            }),
            map(tag("$ ls"), |_| Output::Cmd(Cmd::Ls)),
            map(terminated(tag("dir "), parse_path), |path: &str| {
                Output::Directory(path)
            }),
            map(tuple((number, tag(" "), parse_path)), |(size, _, _)| {
                Output::File(size)
            }),
        )),
    )(input)
}

pub fn make_filesystem(input: &[Output]) -> Filesystem {
    let mut fs = Filesystem::new();
    let mut path = vec![];

    for op in input {
        match op {
            Output::Cmd(Cmd::Cd(CdOpt::Root)) => {
                path.clear();
                path.push(Filesystem::ROOT);
            }
            Output::Cmd(Cmd::Cd(CdOpt::Up)) => {
                path.pop();
            }
            Output::Cmd(Cmd::Cd(CdOpt::Chdir(dir))) => {
                let topfd = path.last().unwrap();
                let fd = fs.open_at(*topfd, dir);
                path.push(fd);
            }
            Output::File(size) => {
                for fd in &path {
                    let node = fs.get_mut(*fd).unwrap();
                    node.size += size;
                }
            }
            _ => {}
        }
    }

    fs
}

pub fn parse_input(input: &str) -> Filesystem {
    let ops = match all_consuming(terminated(parse_output, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap();
    make_filesystem(&ops)
}

pub fn part1(fs: &Filesystem) -> u64 {
    fs.nodes
        .iter()
        .map(|node| node.size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part2(fs: &Filesystem) -> u64 {
    let used = fs.get(Filesystem::ROOT).unwrap().size;
    let free = 70_000_000 - used;
    let needed = 30_000_000 - free;

    fs.nodes
        .iter()
        .map(|node| node.size)
        .filter(|&size| size.checked_sub(needed).is_some())
        .min()
        .unwrap()
}
