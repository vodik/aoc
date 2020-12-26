use crate::parsers::number;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::all_consuming,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    Finish, IResult,
};
use std::cmp::Ordering;
use std::collections::{hash_map::DefaultHasher, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

type Game = (Vec<u32>, Vec<u32>);

fn player(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        delimited(tag("Player "), digit1, tag(":\n")),
        separated_list1(tag("\n"), number),
    )(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    separated_pair(player, tag("\n\n"), player)(input)
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Result<Game, Error<String>> {
    match all_consuming(parse_game)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

fn score_deck(deck: &VecDeque<u32>) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, &card)| card * (idx as u32 + 1))
        .sum()
}

fn combat(card1: u32, card2: u32) -> Player {
    match card1.cmp(&card2) {
        Ordering::Greater => Player::Player1,
        Ordering::Less => Player::Player2,
        Ordering::Equal => panic!("Shouldn't happen in the game"),
    }
}

#[aoc(day22, part1)]
fn part1((deck1, deck2): &Game) -> u32 {
    let mut deck1 = deck1.iter().copied().collect::<VecDeque<_>>();
    let mut deck2 = deck2.iter().copied().collect::<VecDeque<_>>();

    score_deck(&loop {
        match (deck1.get(0), deck2.get(0)) {
            (None, _) => break deck2,
            (_, None) => break deck1,
            (Some(card1), Some(card2)) => match combat(*card1, *card2) {
                Player::Player1 => {
                    deck1.rotate_left(1);
                    deck1.push_back(deck2.pop_front().unwrap());
                }
                Player::Player2 => {
                    deck2.rotate_left(1);
                    deck2.push_back(deck1.pop_front().unwrap());
                }
            },
        }
    })
}

fn hash_game(deck1: &VecDeque<u32>, deck2: &VecDeque<u32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    deck1.hash(&mut hasher);
    deck2.hash(&mut hasher);
    hasher.finish()
}

fn recursive_combat(mut deck1: VecDeque<u32>, mut deck2: VecDeque<u32>) -> (Player, VecDeque<u32>) {
    let mut history = HashSet::new();

    loop {
        if !history.insert(hash_game(&deck1, &deck2)) {
            break (Player::Player1, deck1);
        }

        match (deck1.get(0), deck2.get(0)) {
            (None, _) => break (Player::Player2, deck2),
            (_, None) => break (Player::Player1, deck1),
            (Some(card1), Some(card2)) => {
                let len1 = *card1 as usize;
                let len2 = *card2 as usize;

                let winner = if deck1.len() > len1 && deck2.len() > len2 {
                    recursive_combat(
                        deck1.iter().skip(1).take(len1).copied().collect(),
                        deck2.iter().skip(1).take(len2).copied().collect(),
                    )
                    .0
                } else {
                    combat(*card1, *card2)
                };

                match winner {
                    Player::Player1 => {
                        deck1.rotate_left(1);
                        deck1.push_back(deck2.pop_front().unwrap());
                    }
                    Player::Player2 => {
                        deck2.rotate_left(1);
                        deck2.push_back(deck1.pop_front().unwrap());
                    }
                }
            }
        }
    }
}

#[aoc(day22, part2)]
fn part2((deck1, deck2): &Game) -> u32 {
    score_deck(
        &recursive_combat(
            deck1.iter().copied().collect(),
            deck2.iter().copied().collect(),
        )
        .1,
    )
}
