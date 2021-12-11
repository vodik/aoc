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

type Board = [u16; 25];

#[derive(Debug)]
pub struct Game {
    pub calls: Vec<u16>,
    pub boards: Vec<Board>,
}

#[derive(Debug, Default, Clone, Copy)]
struct BitBoard(u32);

impl BitBoard {
    fn new() -> Self {
        Self::default()
    }

    fn mask(&mut self, mask: u32) {
        self.0 |= mask
    }

    fn has_complete_row(&self) -> bool {
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

        PATTERNS.iter().any(|&pattern| self.0 & pattern == pattern)
    }

    fn score(&self, board: &Board) -> u32 {
        (0..25)
            .filter(|index| self.0 & 1 << index == 0)
            .map(|index| board[index] as u32)
            .sum()
    }
}

#[derive(Debug)]
struct BoardMap([Option<NonZeroU32>; 100]);

impl BoardMap {
    fn from(numbers: &Board) -> Self {
        let mut position_masks = [None; 100];
        for (idx, &number) in numbers.iter().enumerate() {
            position_masks[number as usize] = NonZeroU32::new(1 << u32::try_from(idx).unwrap());
        }

        Self(position_masks)
    }

    fn find(&self, number: u16) -> Option<u32> {
        self.0
            .get(number as usize)
            .and_then(|&mask| mask.map(NonZeroU32::get))
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    preceded(opt(tag(" ")), map_res(digit1, FromStr::from_str))(input)
}

fn board(input: &str) -> IResult<&str, Board> {
    map(
        separated_list1(tag("\n"), separated_list1(tag(" "), number)),
        |numbers: Vec<Vec<u16>>| {
            let numbers: Vec<u16> = numbers.iter().flatten().copied().collect();
            numbers.try_into().unwrap()
        },
    )(input)
}

fn parse_board(input: &str) -> IResult<&str, Vec<Board>> {
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

fn simulate_game(board: &Board, calls: &[u16], limit: usize) -> Option<(usize, u32)> {
    let map = BoardMap::from(board);

    calls
        .iter()
        .take(limit)
        .enumerate()
        .scan(BitBoard::new(), |bb, (generation, &call)| {
            if let Some(mask) = map.find(call) {
                bb.mask(mask);
            }
            Some((generation, call, *bb))
        })
        .find_map(|(generation, call, bb)| {
            bb.has_complete_row()
                .then(|| (generation, bb.score(board) * call as u32))
        })
}

pub fn part1(Game { calls, boards }: &Game) -> u32 {
    let mut limit = boards.len();

    let (_, score) = boards
        .iter()
        .flat_map(|board| {
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
        .flat_map(|board| simulate_game(board, calls, boards.len()))
        .max_by_key(|&(generation, _)| generation)
        .unwrap();

    score
}
