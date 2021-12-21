use std::collections::HashMap;

pub fn parse_input(inpuy: &str) -> (usize, usize) {
    (8, 1)
}

#[derive(Debug, Default)]
struct DeterministicDie(u8, usize);

impl DeterministicDie {
    fn new() -> Self {
        Self::default()
    }

    fn roll(&mut self) -> u16 {
        // triangle number to n - previous roll!
        self.0 = self.0 % 100 + 1;
        self.1 += 1;
        self.0 as u16
    }

    fn count(&self) -> usize {
        self.1
    }
}

const ROLLS: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn part1(input: &(usize, usize)) -> usize {
    let mut die = DeterministicDie::new();

    let mut player1 = 8;
    let mut player1_2 = 0;
    let mut player2 = 1;
    let mut player2_2 = 0;

    for generation in 0.. {
        let rolls = die.roll() + die.roll() + die.roll();
        player1 += rolls;
        player1 %= 10;
        player1_2 += if player1 == 0 { 10 } else { player1 as usize };

        if player1_2 >= 1000 {
            return player2_2 * die.count();
        }

        let rolls = die.roll() + die.roll() + die.roll();
        player2 += rolls;
        player2 %= 10;
        player2_2 += if player2 == 0 { 10 } else { player2 as usize };

        if player2_2 >= 1000 {
            return player1_2 * die.count();
        }
    }

    0
}

pub fn part2(input: &(usize, usize)) -> usize {
    let mut games = HashMap::with_capacity(36);
    for (score, times1) in &ROLLS {
        let mut pos1 = 8 + score;
        pos1 %= 10;
        let points1 = if pos1 == 0 { 10 } else { pos1 as usize };

        for (score, times2) in &ROLLS {
            let mut pos2 = 1 + score;
            pos2 %= 10;
            let points2 = if pos2 == 0 { 10 } else { pos2 as usize };

            games.insert((pos1, pos2, points1, points2), times1 * times2);
        }
    }

    let mut player1 = 0;
    let mut player2 = 0;

    loop {
        let mut games_new = HashMap::with_capacity(10560);

        for ((pos1, pos2, points1, points2), current) in games {
            for (score, times1) in &ROLLS {
                let mut pos1 = pos1 + score;
                pos1 %= 10;
                let points1 = points1 + if pos1 == 0 { 10 } else { pos1 as usize };

                if points1 >= 21 {
                    player1 += current * times1;
                    continue;
                }

                for (score, times2) in &ROLLS {
                    let mut pos2 = pos2 + score;
                    pos2 %= 10;
                    let points2 = points2 + if pos2 == 0 { 10 } else { pos2 as usize };

                    if points2 >= 21 {
                        player2 += current * (times1 * times2);
                        continue;
                    }

                    *games_new.entry((pos1, pos2, points1, points2)).or_default() +=
                        current * (times1 * times2);
                }
            }
        }

        if games_new.is_empty() {
            break;
        }

        games = games_new;
    }

    player1.max(player2)
}
