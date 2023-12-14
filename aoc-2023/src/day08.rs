use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::{all_consuming, map},
    error::Error,
    multi::{fold_many1, many1},
    sequence::{separated_pair, terminated, tuple},
    Finish, IResult,
};

pub type Network = Vec<Option<(u16, u16)>>;

#[derive(Debug)]
pub struct Map {
    steps: Vec<Step>,
    network: Network,
}

impl Map {
    pub fn new(steps: Vec<Step>, network: Network) -> Self {
        Self { steps, network }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Step {
    Left,
    Right,
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    alt((
        map(tag("L"), |_| Step::Left),
        map(tag("R"), |_| Step::Right),
    ))(input)
}

fn parse_steps(input: &str) -> IResult<&str, Vec<Step>> {
    many1(parse_step)(input)
}

fn parse_node(input: &str) -> IResult<&str, u16> {
    map(
        take_while_m_n(3, 3, |c: char| c.is_ascii_uppercase()),
        |node: &str| {
            node.bytes()
                .fold(0, |acc, byte| acc * 26 + (byte - b'A') as u16)
        },
    )(input)
}

fn parse_edge(input: &str) -> IResult<&str, (u16, (u16, u16))> {
    map(
        tuple((
            terminated(parse_node, tag(" = (")),
            terminated(parse_node, tag(", ")),
            terminated(parse_node, tag(")")),
        )),
        |(node, left, right)| (node, (left, right)),
    )(input)
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    fold_many1(
        terminated(parse_edge, tag("\n")),
        || vec![None; 17_576],
        |mut acc, (edge, (left, right))| {
            acc[edge as usize] = Some((left, right));
            acc
        },
    )(input)
}
fn parse_map(input: &str) -> IResult<&str, Map> {
    map(
        separated_pair(parse_steps, tag("\n\n"), parse_network),
        |(steps, network)| Map::new(steps, network),
    )(input)
}

pub fn parse_input(input: &str) -> Map {
    match all_consuming(parse_map)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn traverse(map: &Map, mut position: usize) -> u64 {
    let mut moves = 0;
    while position % 26 != 25 {
        for step in &map.steps {
            let (left, right) = &map.network[position].unwrap();
            position = match step {
                Step::Left => *left as usize,
                Step::Right => *right as usize,
            };
            moves += 1
        }
    }
    moves
}

pub fn part1(input: &Map) -> u64 {
    traverse(input, 0)
}

pub fn part2(input: &Map) -> u64 {
    input
        .network
        .iter()
        .enumerate()
        .filter_map(|(position, &edges)| {
            (position % 26 == 0 && edges.is_some()).then_some(position)
        })
        .fold(None, |max_moves, position| {
            let moves = traverse(input, position);
            match max_moves {
                None => Some(moves),
                Some(max_moves) => Some(lcd(max_moves, moves)),
            }
        })
        .unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcd(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
