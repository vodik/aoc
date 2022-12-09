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
struct Node {
    children: BTreeMap<String, usize>,
    size: u64,
}

#[derive(Debug)]
pub struct Filesystem {
    nodes: Vec<Node>,
}

impl Filesystem {
    const ROOT: usize = 0;

    fn new(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.push(Node::default());
        Filesystem { nodes }
    }

    fn dir_entry(&mut self, inode: usize, path: &str) -> usize {
        let next_inode = self.nodes.len();
        match self.nodes[inode].children.entry(path.into()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(next_inode);
                self.nodes.push(Node::default());
                next_inode
            }
        }
    }

    fn reify_size(&mut self, inode: usize) -> u64 {
        let size: u64 = self.nodes[inode]
            .children
            .values()
            .map(|&inode| self.nodes[inode].size)
            .sum();
        self.nodes[inode].size += size;
        size
    }

    fn get(&self, inode: usize) -> Option<&Node> {
        self.nodes.get(inode)
    }

    fn get_mut(&mut self, inode: usize) -> Option<&mut Node> {
        self.nodes.get_mut(inode)
    }

    fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
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
    let mut fs = Filesystem::new(input.len());
    let mut stack = vec![Filesystem::ROOT];

    for op in &input[1..] {
        match op {
            Output::Cmd(Cmd::Cd(CdOpt::Root)) => {
                todo!();
            }
            Output::Cmd(Cmd::Cd(CdOpt::Up)) => {
                let inode = stack.pop().unwrap();
                fs.reify_size(inode);
            }
            Output::Cmd(Cmd::Cd(CdOpt::Chdir(dir))) => {
                let inode = stack.last().unwrap();
                stack.push(fs.dir_entry(*inode, dir));
            }
            Output::File(size) => {
                let inode = stack.last().unwrap();
                fs.get_mut(*inode).unwrap().size += *size;
            }
            _ => {}
        }
    }

    while let Some(inode) = stack.pop() {
        fs.reify_size(inode);
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
    fs.nodes()
        .map(|node| node.size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part2(fs: &Filesystem) -> u64 {
    let used = fs.get(Filesystem::ROOT).unwrap().size;
    let free = 70_000_000 - used;
    let needed = 30_000_000 - free;

    fs.nodes()
        .map(|node| node.size)
        .filter(|&size| size >= needed)
        .min()
        .unwrap()
}
