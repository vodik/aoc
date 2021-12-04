use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Game {
    pub calls: Vec<u16>,
    pub boards: Vec<Board>,
}

#[derive(Debug, Clone)]
pub struct Board {
    layout: [u16; 25],
    set: [bool; 25],
}

impl Board {
    fn call(&mut self, number: u16) {
        if let Some(idx) = self.layout.iter().position(|&v| v == number) {
            self.set[idx] = true;
        }
    }

    fn won(&self) -> bool {
        const PATTERNS: [[usize; 5]; 10] = [
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
        ];

        PATTERNS
            .iter()
            .any(|&pattern| pattern.iter().all(|&idx| self.set[idx]))
    }

    fn score(&self) -> u32 {
        self.set
            .iter()
            .enumerate()
            .filter(|(_, &s)| !s)
            .map(|(idx, _)| self.layout[idx] as u32)
            .sum()
    }
}

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    preceded(opt(tag(" ")), map_res(digit1, FromStr::from_str))(input)
}

fn board(input: &str) -> IResult<&str, Board> {
    map(
        separated_list1(tag("\n"), separated_list1(tag(" "), number)),
        |numbers: Vec<Vec<u16>>| {
            let numbers = numbers.into_iter().flatten().collect::<Vec<u16>>();
            Board {
                layout: numbers.try_into().unwrap(),
                set: [false; 25],
            }
        },
    )(input)
}

fn parse_board(input: &str) -> IResult<&str, Vec<Board>> {
    separated_list1(tag("\n\n"), board)(input)
}

fn parse_input2(input: &str) -> IResult<&str, Game> {
    map(
        terminated(
            separated_pair(separated_list1(tag(","), number), tag("\n\n"), parse_board),
            tag("\n"),
        ),
        |(calls, boards)| Game { calls, boards },
    )(input)
}

pub fn parse_input(input: &str) -> Game {
    match all_consuming(parse_input2)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &Game) -> u32 {
    let mut input = input.clone();
    let mut winner = None;
    let mut called = None;

    for call in input.calls {
        for board in input.boards.iter_mut() {
            board.call(call);
        }

        let mut won: Vec<Board> = input.boards.drain_filter(|board| board.won()).collect();
        if !won.is_empty() {
            winner = won.pop();
            called = Some(call as u32);
            break;
        }
    }

    winner.unwrap().score() * called.unwrap()
}

pub fn part2(input: &Game) -> u32 {
    let mut input = input.clone();
    let mut winner = None;
    let mut called = None;

    for call in input.calls {
        for board in input.boards.iter_mut() {
            board.call(call);
        }

        let mut won: Vec<Board> = input.boards.drain_filter(|board| board.won()).collect();
        if input.boards.is_empty() {
            winner = won.pop();
            called = Some(call as u32);
            break;
        }
    }

    winner.unwrap().score() * called.unwrap()
}
