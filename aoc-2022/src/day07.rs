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
use std::{
    collections::{btree_map::Entry, BTreeMap},
    str::FromStr,
};

#[derive(Debug, Default)]
struct Metadata {
    size: u64,
}

#[derive(Debug)]
pub struct Filesystem {
    nodes: Vec<Metadata>,
    entries: BTreeMap<(usize, String), usize>,
}

impl Filesystem {
    const ROOT: usize = 0;

    fn new() -> Self {
        Filesystem {
            nodes: vec![Metadata::default()],
            entries: BTreeMap::new(),
        }
    }

    fn dir_entry(&mut self, inode: usize, path: &str) -> usize {
        let next_inode = self.nodes.len();
        match self.entries.entry((inode, path.into())) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(next_inode);
                self.nodes.push(Default::default());
                next_inode
            }
        }
    }

    fn get(&self, inode: usize) -> Option<&Metadata> {
        self.nodes.get(inode)
    }

    fn get_mut(&mut self, inode: usize) -> Option<&mut Metadata> {
        self.nodes.get_mut(inode)
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

pub fn make_filesystem(input: &[Output]) -> Filesystem {
    let mut fs = Filesystem::new();
    let mut stack = vec![];

    for op in input {
        match op {
            Output::Cmd(Cmd::Cd(CdOpt::Root)) => {
                stack.clear();
                stack.push(Filesystem::ROOT);
            }
            Output::Cmd(Cmd::Cd(CdOpt::Up)) => {
                stack.pop();
            }
            Output::Cmd(Cmd::Cd(CdOpt::Chdir(dir))) => {
                let top_inode = stack.last().unwrap();
                stack.push(fs.dir_entry(*top_inode, dir));
            }
            Output::File(size) => {
                for &inode in &stack {
                    let dir_entry = fs.get_mut(inode).unwrap();
                    dir_entry.size += size;
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
        .map(|dir_entry| dir_entry.size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part2(fs: &Filesystem) -> u64 {
    let used = fs.get(Filesystem::ROOT).unwrap().size;
    let free = 70_000_000 - used;
    let needed = 30_000_000 - free;

    fs.nodes
        .iter()
        .map(|dir_entry| dir_entry.size)
        .filter(|&size| size >= needed)
        .min()
        .unwrap()
}
