use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res, opt, recognize},
    error::Error,
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use std::str::FromStr;

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), FromStr::from_str)(input)
}

fn parse_rule(input: &str) -> IResult<&str, u16> {
    preceded(
        tuple((tag("Player "), digit1, tag(" starting position: "))),
        number,
    )(input)
}

fn parse_state(input: &str) -> IResult<&str, (u16, u16)> {
    separated_pair(parse_rule, tag("\n"), parse_rule)(input)
}

pub fn parse_input(input: &str) -> (u16, u16) {
    match all_consuming(terminated(parse_state, tag("\n")))(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

#[derive(Debug, Default)]
struct DeterministicDie {
    state: u16,
    rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self::default()
    }

    fn roll(&mut self) -> u16 {
        self.rolls += 3;
        if self.state + 3 <= 100 {
            let roll = 3 * self.state + 6; // T_n + 3 - T_n
            self.state += 3;
            roll
        } else {
            let mut roll = 0;
            for _ in 0..3 {
                self.state = self.state % 100 + 1;
                roll += self.state;
            }
            roll
        }
    }

    fn count(&self) -> usize {
        self.rolls
    }
}

#[derive(Clone, Copy)]
struct Player {
    position: u16,
    score: u16,
}

impl Player {
    fn new(position: u16) -> Self {
        Self { position, score: 0 }
    }

    fn advance(&mut self, rolls: u16) {
        self.position = (self.position + rolls - 1) % 10 + 1;
        self.score += self.position;
    }

    fn score(&self) -> usize {
        self.score as usize
    }
}

struct State(Player, Player);

impl State {
    fn pack(&self) -> usize {
        ((self.0.score * 10 + self.0.position) as usize & 0b11111111) << 8
            | ((self.1.score * 10 + self.1.position) as usize & 0b11111111)
    }

    fn unpack(index: usize) -> Self {
        let player1 = ((index >> 8) & 0b11111111) as u16;
        let player2 = (index & 0b11111111) as u16;

        Self(
            Player {
                position: player1 % 10,
                score: player1 / 10,
            },
            Player {
                position: player2 % 10,
                score: player2 / 10,
            },
        )
    }
}

pub fn part1(input: &(u16, u16)) -> usize {
    let mut die = DeterministicDie::new();

    let mut player1 = Player::new(input.0);
    let mut player2 = Player::new(input.1);

    for _ in 0.. {
        let rolls = die.roll();
        player1.advance(rolls);

        if player1.score() >= 1000 {
            return player2.score() * die.count();
        }

        let rolls = die.roll();
        player2.advance(rolls);

        if player2.score() >= 1000 {
            return player2.score() * die.count();
        }
    }

    0
}

pub fn part2(input: &(u16, u16)) -> usize {
    const ROLLS: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut counter = [0usize; 0xd2d3];

    for (roll, wins) in &ROLLS {
        let mut player1 = Player::new(input.0);
        player1.advance(*roll);

        for (roll, times) in &ROLLS {
            let wins = wins * times;
            let mut player2 = Player::new(input.1);
            player2.advance(*roll);

            let state = State(player1, player2);
            counter[state.pack()] += wins;
        }
    }

    let mut wins1 = 0;
    let mut wins2 = 0;

    loop {
        let mut dirty = false;
        let mut next = [0usize; 0xd2d3];

        for (index, &wins) in counter.iter().enumerate() {
            if wins == 0 {
                continue;
            }

            let State(player1, player2) = State::unpack(index);

            for (roll, times) in &ROLLS {
                let wins = wins * times;
                let mut player1 = player1;
                player1.advance(*roll);

                if player1.score() >= 21 {
                    wins1 += wins;
                    continue;
                }

                for (roll, times) in &ROLLS {
                    let wins = wins * times;
                    let mut player2 = player2;
                    player2.advance(*roll);

                    if player2.score() >= 21 {
                        wins2 += wins;
                        continue;
                    }

                    let state = State(player1, player2);
                    next[state.pack()] += wins;
                    dirty = true;
                }
            }
        }

        if !dirty {
            break wins1.max(wins2);
        }

        counter = next;
    }
}
