use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
pub enum Cipher {
    X,
    Y,
    Z,
}

impl Play {
    fn score_play(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn score_game(&self, play: &Self) -> u32 {
        play.score_play()
            + match (self, play) {
                (Play::Rock, Play::Rock) => 3,
                (Play::Rock, Play::Paper) => 6,
                (Play::Rock, Play::Scissors) => 0,
                (Play::Paper, Play::Rock) => 0,
                (Play::Paper, Play::Paper) => 3,
                (Play::Paper, Play::Scissors) => 6,
                (Play::Scissors, Play::Rock) => 6,
                (Play::Scissors, Play::Paper) => 0,
                (Play::Scissors, Play::Scissors) => 3,
            }
    }
}

fn parse_line(input: &str) -> IResult<&str, (Play, Cipher)> {
    separated_pair(
        map(one_of("ABC"), |c| match c {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            _ => unreachable!(),
        }),
        tag(" "),
        map(one_of("XYZ"), |c| match c {
            'X' => Cipher::X,
            'Y' => Cipher::Y,
            'Z' => Cipher::Z,
            _ => unreachable!(),
        }),
    )(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(Play, Cipher)>> {
    separated_list1(tag("\n"), parse_line)(input)
}

pub fn parse_input(input: &str) -> Vec<(Play, Cipher)> {
    match all_consuming(terminated(parse_lines, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &[(Play, Cipher)]) -> u32 {
    input.iter().fold(0, |score, &(challenge, cipher)| {
        let decoded_play = match cipher {
            Cipher::X => Play::Rock,
            Cipher::Y => Play::Paper,
            Cipher::Z => Play::Scissors,
        };

        score + challenge.score_game(&decoded_play)
    })
}

pub fn part2(input: &[(Play, Cipher)]) -> u32 {
    input.iter().fold(0, |score, &(challenge, cipher)| {
        let decoded_play = match cipher {
            Cipher::X => match challenge {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            Cipher::Y => challenge,
            Cipher::Z => match challenge {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
        };

        score + challenge.score_game(&decoded_play)
    })
}
