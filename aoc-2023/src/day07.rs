use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    fn new(cards: [u8; 5], bid: u32) -> Self {
        Self { cards, bid }
    }

    fn score(&self, score: fn(u8) -> Option<u8>) -> ScoredHand {
        ScoredHand::new(self, score)
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_cards(input: &str) -> IResult<&str, [u8; 5]> {
    map(
        take_while_m_n(5, 5, |c: char| c.is_ascii_alphanumeric()),
        |cards: &str| cards.bytes().collect::<Vec<_>>().try_into().unwrap(),
    )(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map(
        separated_pair(parse_cards, space1, number),
        |(cards, bid)| Hand::new(cards, bid),
    )(input)
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    terminated(separated_list1(tag("\n"), parse_hand), tag("\n"))(input)
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    match all_consuming(parse_hands)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScoredHand {
    cards: [u8; 5],
    score: Score,
    bid: u32,
}

impl ScoredHand {
    pub fn new(hand: &Hand, score: fn(u8) -> Option<u8>) -> Self {
        let mut cards = [0; 5];
        let mut counts = [0; 14];

        for (slot, &card) in cards.iter_mut().zip(hand.cards.iter()) {
            *slot = score(card).unwrap();
            counts[*slot as usize] += 1;
        }

        let mut jokers = 0;
        std::mem::swap(&mut jokers, &mut counts[0]);
        counts.sort_by(|a, b| b.cmp(a));
        counts[0] += jokers;

        let score = match counts[..2] {
            [5, _] => Score::FiveOfAKind,
            [4, _] => Score::FourOfAKind,
            [3, 2] => Score::FullHouse,
            [3, _] => Score::ThreeOfAKind,
            [2, 2] => Score::TwoPair,
            [2, _] => Score::OnePair,
            _ => Score::HighCard,
        };

        Self {
            cards,
            score,
            bid: hand.bid,
        }
    }
}

impl PartialOrd for ScoredHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoredHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .cmp(&other.score)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

pub fn part1(input: &[Hand]) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .map(|hand| {
            hand.score(|card| match card {
                b'2' => Some(1),
                b'3' => Some(2),
                b'4' => Some(3),
                b'5' => Some(4),
                b'6' => Some(5),
                b'7' => Some(6),
                b'8' => Some(7),
                b'9' => Some(8),
                b'T' => Some(9),
                b'J' => Some(10),
                b'Q' => Some(11),
                b'K' => Some(12),
                b'A' => Some(13),
                _ => None,
            })
        })
        .collect();

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum()
}

pub fn part2(input: &[Hand]) -> usize {
    let mut hands: Vec<_> = input
        .iter()
        .map(|hand| {
            hand.score(|card| match card {
                b'J' => Some(0),
                b'2' => Some(1),
                b'3' => Some(2),
                b'4' => Some(3),
                b'5' => Some(4),
                b'6' => Some(5),
                b'7' => Some(6),
                b'8' => Some(7),
                b'9' => Some(8),
                b'T' => Some(9),
                b'Q' => Some(10),
                b'K' => Some(11),
                b'A' => Some(12),
                _ => None,
            })
        })
        .collect();

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum()
}
