// use nom::{
//     branch::alt,
//     bytes::complete::tag,
//     character::complete::digit1,
//     combinator::{all_consuming, map, map_res, opt, recognize},
//     error::Error,
//     multi::separated_list1,
//     sequence::{preceded, separated_pair, terminated, tuple},
//     Finish, IResult,
// };
// use std::{
//     cmp::min,
//     collections::{HashMap, HashSet},
//     ops::Range,
//     str::FromStr,
// };

#[derive(Debug, Clone, Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

pub fn parse_input(input: &str) -> () {
    ()
    // match all_consuming(terminated(parse_ops, tag("\n")))(input).finish() {
    //     Ok((_, output)) => Ok(output),
    //     Err(Error { input, code }) => Err(Error {
    //         input: input.to_string(),
    //         code,
    //     }),
    // }
    // .unwrap();
}

const ENTER_ENERGY: usize = 3333;
const ENTER_ENERGY2: usize = 11110;

#[derive(Debug, Clone, Copy)]
struct Burrow {
    amber: u32,
    bronze: u32,
    copper: u32,
    desert: u32,
}

const HALLWAY_MASK: u32 = 0b1111111;

impl Burrow {
    fn new(rooms: &[&[Amphipod]; 4]) -> Self {
        let mut amber = 0;
        let mut bronze = 0;
        let mut copper = 0;
        let mut desert = 0;

        for (room, members) in rooms.iter().rev().enumerate() {
            for (idx, pod) in members.iter().enumerate() {
                let mask = 1 << (idx + 4 * room + 7) as u32;
                match pod {
                    Amphipod::Amber => amber |= mask,
                    Amphipod::Bronze => bronze |= mask,
                    Amphipod::Copper => copper |= mask,
                    Amphipod::Desert => desert |= mask,
                };
            }
        }

        Self {
            amber,
            bronze,
            copper,
            desert,
        }
    }

    fn flatten(&self) -> u32 {
        self.amber | self.bronze | self.copper | self.desert
    }

    fn hallway(&self) -> u32 {
        self.flatten() & HALLWAY_MASK
    }

    fn room(&self, pos: usize) -> u32 {
        let shift = (3 - pos) * 4 + 7;
        self.flatten() >> shift & 0b1111
    }

    fn is_room_empty(&self, pos: usize) -> bool {
        self.room(pos) == 0
    }

    fn is_empty(&self) -> bool {
        self.flatten() >> 7 == 0
    }

    fn has_path(&self, a: usize, b: usize) -> bool {
        let hallway = self.hallway();
        let start = a.min(b);
        let end = a.max(b);

        for pos in start..end + 1 {
            let mask = 1 << (pos * 2 + 2);
            if hallway & mask == mask {
                return false;
            }
        }
        true
    }

    fn commit(&mut self, (pos, pod, mask1, mask2, _): (usize, Amphipod, u32, u32, usize)) {
        match pod {
            Amphipod::Amber => {
                self.amber &= !mask1;
                self.amber |= mask2;
            }
            Amphipod::Bronze => {
                self.bronze &= !mask1;
                self.bronze |= mask2;
            }
            Amphipod::Copper => {
                self.copper &= !mask1;
                self.copper |= mask2;
            }
            Amphipod::Desert => {
                self.desert &= !mask1;
                self.desert |= mask2;
            }
        }

        if pos == 0 && self.is_room_empty(0) && self.amber < 127 {
            self.amber = 0;
        }
        if pos == 1 && self.is_room_empty(1) && self.bronze < 127 {
            self.bronze = 0;
        }
        if pos == 2 && self.is_room_empty(2) && self.copper < 127 {
            self.copper = 0;
        }
        if pos == 3 && self.is_room_empty(3) && self.desert < 127 {
            self.desert = 0;
        }
    }

    // Move(
    //    pod, mask  -  for removal
    //    hallway position
    //    simulated cost
    fn moves(&self, pos: usize) -> Vec<(usize, Amphipod, u32, u32, usize)> {
        let mut moves = vec![];

        let room = self.room(pos);
        if room == 0 {
            return moves;
        }

        let mut mask = None;
        for idx in 0..4 {
            let m = 1 << idx;
            if room & m == m {
                mask = Some(idx);
                break;
            }
        }

        if mask.is_none() {
            return moves;
        }

        let mb = (3 - pos) * 4 + 7;
        let mask = 1 << (mb + mask.unwrap());
        let pod = if self.amber & mask == mask {
            Amphipod::Amber
        } else if self.bronze & mask == mask {
            Amphipod::Bronze
        } else if self.copper & mask == mask {
            Amphipod::Copper
        } else if self.desert & mask == mask {
            Amphipod::Desert
        } else {
            return moves;
        };

        let target = match pod {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        };

        if !self.has_path(pos, target) {
            return moves;
        }

        let basecost = abs(pos, target) * 2;
        if self.is_room_empty(target) {
            moves.push((pos, pod, mask, 0, basecost * pod.energy()));
            return moves;
        }

        // left
        match pos.min(target) {
            0 => {
                let mask2 = 0b0100000;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b1000000;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 4) * pod.energy()));
                    }
                }
            }
            1 => {
                let mask2 = 0b0010000;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0100000;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 6) * pod.energy()));
                        let mask2 = 0b1000000;
                        if mask2 & self.hallway() == 0 {
                            moves.push((pos, pod, mask, mask2, (basecost + 8) * pod.energy()));
                        }
                    }
                }
            }
            2 => {
                let mask2 = 0b0001000;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0010000;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 6) * pod.energy()));
                        let mask2 = 0b0100000;
                        if mask2 & self.hallway() == 0 {
                            moves.push((pos, pod, mask, mask2, (basecost + 10) * pod.energy()));
                            let mask2 = 0b1000000;
                            if mask2 & self.hallway() == 0 {
                                moves.push((pos, pod, mask, mask2, (basecost + 12) * pod.energy()));
                            }
                        }
                    }
                }
            }
            3 => {
                let mask2 = 0b0000100;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0001000;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 6) * pod.energy()));
                        let mask2 = 0b0010000;
                        if mask2 & self.hallway() == 0 {
                            moves.push((pos, pod, mask, mask2, (basecost + 10) * pod.energy()));
                            let mask2 = 0b0100000;
                            if mask2 & self.hallway() == 0 {
                                moves.push((pos, pod, mask, mask2, (basecost + 14) * pod.energy()));
                                let mask2 = 0b1000000;
                                if mask2 & self.hallway() == 0 {
                                    moves.push((
                                        pos,
                                        pod,
                                        mask,
                                        mask2,
                                        (basecost + 16) * pod.energy(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        // right
        match pos.max(target) {
            3 => {
                let mask2 = 0b0000010;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0000001;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 4) * pod.energy()));
                    }
                }
            }
            2 => {
                let mask2 = 0b0000100;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0000010;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 6) * pod.energy()));
                        let mask2 = 0b0000001;
                        if mask2 & self.hallway() == 0 {
                            moves.push((pos, pod, mask, mask2, (basecost + 8) * pod.energy()));
                        }
                    }
                }
            }
            1 => {
                let mask2 = 0b0001000;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0000100;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 6) * pod.energy()));
                        let mask2 = 0b0000010;
                        if mask2 & self.hallway() == 0 {
                            moves.push((pos, pod, mask, mask2, (basecost + 10) * pod.energy()));
                            let mask2 = 0b0000001;
                            if mask2 & self.hallway() == 0 {
                                moves.push((pos, pod, mask, mask2, (basecost + 12) * pod.energy()));
                            }
                        }
                    }
                }
            }
            0 => {
                let mask2 = 0b0010000;
                if mask2 & self.hallway() == 0 {
                    moves.push((pos, pod, mask, mask2, (basecost + 2) * pod.energy()));
                    let mask2 = 0b0001000;
                    if mask2 & self.hallway() == 0 {
                        moves.push((pos, pod, mask, mask2, (basecost + 6) * pod.energy()));
                        let mask2 = 0b0000100;
                        if mask2 & self.hallway() == 0 {
                            moves.push((pos, pod, mask, mask2, (basecost + 10) * pod.energy()));
                            let mask2 = 0b0000010;
                            if mask2 & self.hallway() == 0 {
                                moves.push((pos, pod, mask, mask2, (basecost + 14) * pod.energy()));
                                let mask2 = 0b0000001;
                                if mask2 & self.hallway() == 0 {
                                    moves.push((
                                        pos,
                                        pod,
                                        mask,
                                        mask2,
                                        (basecost + 16) * pod.energy(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        moves
    }
}

fn abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn solve(rooms: &[&[Amphipod]; 4]) -> usize {
    let mut stack = vec![(Burrow::new(rooms), 0)];
    let mut min = usize::MAX;

    while let Some((burrow, cost)) = stack.pop() {
        for t in (0..4).rev() {
            let futures = burrow.moves(t);
            for future in futures {
                let mut burrow = burrow;
                let cost = cost + future.4;
                if cost < min {
                    burrow.commit(future);
                    if burrow.is_empty() {
                        min = cost;
                    } else {
                        stack.push((burrow, cost));
                    }
                }
            }
        }
    }

    min
}

pub fn part1(input: &()) -> usize {
    let rooms: [&[Amphipod]; 4] = [
        &[Amphipod::Amber, Amphipod::Desert],
        &[Amphipod::Copper, Amphipod::Desert],
        &[Amphipod::Bronze, Amphipod::Bronze],
        &[Amphipod::Amber, Amphipod::Copper],
    ];

    let exit_energy: usize = rooms
        .iter()
        .flat_map(|room| {
            room.iter()
                .enumerate()
                .map(|(idx, pod)| (idx + 1) * pod.energy())
        })
        .sum();

    let transit_energy = solve(&rooms);
    ENTER_ENERGY + exit_energy + transit_energy
}

pub fn part2(input: &()) -> usize {
    let rooms: [&[Amphipod]; 4] = [
        &[
            Amphipod::Amber,
            Amphipod::Desert,
            Amphipod::Desert,
            Amphipod::Desert,
        ],
        &[
            Amphipod::Copper,
            Amphipod::Copper,
            Amphipod::Bronze,
            Amphipod::Desert,
        ],
        &[
            Amphipod::Bronze,
            Amphipod::Bronze,
            Amphipod::Amber,
            Amphipod::Bronze,
        ],
        &[
            Amphipod::Amber,
            Amphipod::Amber,
            Amphipod::Copper,
            Amphipod::Copper,
        ],
    ];

    let exit_energy: usize = rooms
        .iter()
        .flat_map(|room| {
            room.iter()
                .enumerate()
                .map(|(idx, pod)| (idx + 1) * pod.energy())
        })
        .sum();

    let transit_energy = solve(&rooms);
    ENTER_ENERGY2 + exit_energy + transit_energy
}
