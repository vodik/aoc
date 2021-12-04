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

#[derive(Debug)]
pub struct Game {
    pub calls: Vec<u16>,
    pub boards: Vec<[u16; 25]>,
}

#[derive(Debug)]
pub struct Board {
    numbers: [u16; 25],
    positions: [isize; 100],
    state: [bool; 25],
}

impl Board {
    pub fn new(numbers: [u16; 25]) -> Self {
        let mut positions = [-1; 100];
        for (idx, &number) in numbers.iter().enumerate() {
            positions[number as usize] = isize::try_from(idx).unwrap();
        }

        Self {
            numbers,
            positions,
            state: Default::default(),
        }
    }

    fn advance(&mut self, number: u16) -> Option<u32> {
        self.call(number);
        self.won().then(|| self.score())
    }

    fn call(&mut self, number: u16) {
        if let Some(&idx) = self.positions.get(number as usize) {
            if !idx.is_negative() {
                self.state[idx as usize] = true;
            }
        }
    }

    fn won(&self) -> bool {
        const PATTERNS: &[[usize; 5]; 10] = &[
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
            .any(|&pattern| pattern.iter().all(|&idx| self.state[idx]))
    }

    fn score(&self) -> u32 {
        self.state
            .iter()
            .enumerate()
            .filter(|(_, &state)| !state)
            .map(|(idx, _)| self.numbers[idx] as u32)
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

fn simulate_game(board: [u16; 25], calls: &[u16]) -> Option<(usize, u32)> {
    let mut board = Board::new(board);

    calls.iter().enumerate().find_map(|(generation, &call)| {
        let score = board.advance(call)?;
        Some((generation, call as u32 * score))
    })
}

pub fn part1(Game { calls, boards }: &Game) -> u32 {
    let (_, score) = boards
        .iter()
        .flat_map(|&board| simulate_game(board, calls))
        .min_by_key(|&(generation, _)| generation)
        .unwrap();

    score
}

pub fn part2(Game { calls, boards }: &Game) -> u32 {
    let (_, score) = boards
        .iter()
        .flat_map(|&board| simulate_game(board, calls))
        .max_by_key(|&(generation, _)| generation)
        .unwrap();

    score
}
