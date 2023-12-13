use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::{fold_many0, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    winners: u128,
    numbers: u128,
}

impl Card {
    pub fn new(winners: u128, numbers: u128) -> Self {
        Self { winners, numbers }
    }

    pub fn matches(&self) -> usize {
        (self.winners & self.numbers).count_ones() as _
    }

    pub fn score(&self) -> u32 {
        match self.matches() {
            0 => 0,
            matches => 1 << (matches - 1),
        }
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, u128> {
    fold_many0(
        preceded(space1, number::<u8>),
        || 0,
        |acc, number| acc | 1 << number,
    )(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(
        preceded(
            tuple((tag("Card"), space1, digit1, tag(":"))),
            separated_pair(parse_numbers, tag(" |"), parse_numbers),
        ),
        |(winners, numbers)| Card::new(winners, numbers),
    )(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    terminated(separated_list1(tag("\n"), parse_card), tag("\n"))(input)
}

pub fn parse_input(input: &str) -> Vec<Card> {
    match all_consuming(parse_cards)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[Card]) -> u32 {
    input.iter().map(Card::score).sum()
}

pub fn part2(input: &[Card]) -> u32 {
    let mut clones = [1u32; 187];
    for (game, card) in input.iter().enumerate() {
        let bonus = clones[game];
        let matches = card.matches();
        for future_game in clones.iter_mut().take(game + matches + 1).skip(game + 1) {
            *future_game += bonus;
        }
    }
    clones.iter().sum()
}
