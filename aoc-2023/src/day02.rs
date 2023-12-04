use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    error::Error,
    multi::{fold_many0, separated_list1},
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Debug)]
enum Rule {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, Default)]
pub struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn merge(&self, other: &Set) -> Set {
        Set {
            red: u32::max(self.red, other.red),
            green: u32::max(self.green, other.green),
            blue: u32::max(self.blue, other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((
        map(terminated(number, tag(" red")), Rule::Red),
        map(terminated(number, tag(" green")), Rule::Green),
        map(terminated(number, tag(" blue")), Rule::Blue),
    ))(input)
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    fold_many0(
        terminated(parse_rule, opt(tag(", "))),
        Set::default,
        |mut set, rule| {
            match rule {
                Rule::Red(count) => set.red = count,
                Rule::Green(count) => set.green = count,
                Rule::Blue(count) => set.blue = count,
            };
            set
        },
    )(input)
}

fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
    separated_list1(tag("; "), parse_set)(input)
}

fn parse_game(input: &str) -> IResult<&str, Vec<Set>> {
    preceded(tuple((tag("Game "), digit1, tag(": "))), parse_sets)(input)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Vec<Set>>> {
    terminated(separated_list1(tag("\n"), parse_game), opt(tag("\n")))(input)
}

pub fn parse_input(input: &str) -> Vec<Vec<Set>> {
    match all_consuming(parse_games)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[Vec<Set>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, game)| {
            let valid = !game
                .iter()
                .any(|game| game.red > 12 || game.green > 13 || game.blue > 14);
            valid.then_some(index + 1)
        })
        .sum()
}

pub fn part2(input: &[Vec<Set>]) -> u32 {
    input
        .iter()
        .map(|game| {
            game.iter()
                .fold(Set::default(), |acc, set| acc.merge(set))
                .power()
        })
        .sum()
}
