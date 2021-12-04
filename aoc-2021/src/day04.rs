use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::{num::NonZeroU32, str::FromStr};

#[derive(Debug)]
pub struct Game {
    pub calls: Vec<u16>,
    pub boards: Vec<[u16; 25]>,
}

#[derive(Debug)]
pub struct Board {
    numbers: [u16; 25],
    position_masks: [Option<NonZeroU32>; 100],
    state: u32,
}

impl Board {
    fn new(numbers: [u16; 25]) -> Self {
        let mut position_masks = [None; 100];
        for (idx, &number) in numbers.iter().enumerate() {
            position_masks[number as usize] =
                Some(NonZeroU32::new(1 << u32::try_from(idx).unwrap()).unwrap());
        }

        Self {
            numbers,
            position_masks,
            state: Default::default(),
        }
    }

    fn advance(&mut self, number: u16) -> Option<u32> {
        self.call(number);
        self.won().then(|| self.score())
    }

    fn call(&mut self, number: u16) {
        if let Some(&Some(mask)) = self.position_masks.get(number as usize) {
            self.state |= mask.get();
        }
    }

    fn won(&self) -> bool {
        const PATTERNS: &[u32; 10] = &[
            0b0000000000000000000011111,
            0b0000000000000001111100000,
            0b0000000000111110000000000,
            0b0000011111000000000000000,
            0b1111100000000000000000000,
            0b0000100001000010000100001,
            0b0001000010000100001000010,
            0b0010000100001000010000100,
            0b0100001000010000100001000,
            0b1000010000100001000010000,
        ];

        PATTERNS
            .iter()
            .any(|&pattern| self.state & pattern == pattern)
    }

    fn score(&self) -> u32 {
        (0..25)
            .filter(|index| self.state & 1 << index == 0)
            .map(|index| self.numbers[index] as u32)
            .sum()
    }
}

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    preceded(opt(tag(" ")), map_res(digit1, FromStr::from_str))(input)
}

fn board(input: &str) -> IResult<&str, [u16; 25]> {
    map(
        separated_list1(tag("\n"), separated_list1(tag(" "), number)),
        |numbers: Vec<Vec<u16>>| {
            let numbers = numbers.into_iter().flatten().collect::<Vec<u16>>();
            numbers.try_into().unwrap()
        },
    )(input)
}

fn parse_board(input: &str) -> IResult<&str, Vec<[u16; 25]>> {
    separated_list1(tag("\n\n"), board)(input)
}

fn parse_calls(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(tag(","), number)(input)
}

fn parse_file(input: &str) -> IResult<&str, Game> {
    map(
        terminated(
            separated_pair(parse_calls, tag("\n\n"), parse_board),
            tag("\n"),
        ),
        |(calls, boards)| Game { calls, boards },
    )(input)
}

pub fn parse_input(input: &str) -> Game {
    match all_consuming(parse_file)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn simulate_game(board: [u16; 25], calls: &[u16], limit: usize) -> Option<(usize, u32)> {
    let mut board = Board::new(board);

    calls
        .iter()
        .take(limit)
        .enumerate()
        .find_map(|(generation, &call)| {
            let score = board.advance(call)?;
            Some((generation, call as u32 * score))
        })
}

pub fn part1(Game { calls, boards }: &Game) -> u32 {
    let mut limit = boards.len();

    let (_, score) = boards
        .iter()
        .flat_map(|&board| {
            let (generation, score) = simulate_game(board, calls, limit)?;
            limit = limit.min(generation);
            Some((generation, score))
        })
        .min_by_key(|&(generation, _)| generation)
        .unwrap();

    score
}

pub fn part2(Game { calls, boards }: &Game) -> u32 {
    let (_, score) = boards
        .iter()
        .flat_map(|&board| simulate_game(board, calls, boards.len()))
        .max_by_key(|&(generation, _)| generation)
        .unwrap();

    score
}
