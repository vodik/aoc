#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    const fn energy(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

pub fn parse_input(_input: &str) {
    // match all_consuming(terminated(parse_ops, tag("\n")))(input).finish() {
    //     Ok((_, output)) => Ok(output),
    //     Err(Error { input, code }) => Err(Error {
    //         input: input.to_string(),
    //         code,
    //     }),
    // }
    // .unwrap();
}

const fn entry_energy(n: usize) -> usize {
    let base = n * (n + 1) / 2;
    base + base * 10 + base * 100 + base * 1000
}

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

        let mask = (start..end).fold(0, |acc, pos| acc | 1 << 4 - pos);
        hallway & mask == 0
    }

    fn commit(&mut self, pod: Amphipod, (mask1, mask2, _): (u32, u32, usize)) {
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

        if self.is_room_empty(0) {
            self.amber &= !HALLWAY_MASK;
        }
        if self.is_room_empty(1) {
            self.bronze &= !HALLWAY_MASK;
        }
        if self.is_room_empty(2) {
            self.copper &= !HALLWAY_MASK;
        }
        if self.is_room_empty(3) {
            self.desert &= !HALLWAY_MASK;
        }
    }

    fn moves(&self, pos: usize) -> Option<(Amphipod, Vec<(u32, u32, usize)>)> {
        let room = self.room(pos);
        if room == 0 {
            return None;
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
            return None;
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
            return None;
        };

        let target = match pod {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        };

        const COSTS: [usize; 7] = [2, 2, 4, 4, 4, 2, 2];
        let hallway = self.hallway();
        let mut moves = Vec::with_capacity(7);
        let basecost = abs(target, pos) * 2;

        if !self.has_path(pos, target) {
            // START HACK
            let mut first = true;
            if pos < target {
                let end = pos.min(target);
                moves.extend((0..end + 2).rev().scan(basecost, |inc, offset| {
                    let mask2 = 1u32 << (6 - offset);
                    let c = if first {
                        let r = *inc + 2;
                        *inc += COSTS[offset];
                        first = false;
                        r
                    } else {
                        *inc += COSTS[offset];
                        *inc
                    };
                    (mask2 & hallway == 0).then(|| (mask, mask2, c * pod.energy()))
                }));
            } else {
                let start = pos.max(target);
                moves.extend((start + 2..7).scan(basecost, |inc, offset| {
                    let mask2 = 1u32 << (6 - offset);
                    let c = if first {
                        let r = *inc + 2;
                        *inc += COSTS[offset];
                        first = false;
                        r
                    } else {
                        *inc += COSTS[offset];
                        *inc
                    };
                    (mask2 & hallway == 0).then(|| (mask, mask2, c * pod.energy()))
                }));
            }

            return (!moves.is_empty()).then(|| (pod, moves));
        }

        if self.is_room_empty(target) {
            return Some((pod, vec![(mask, 0, basecost * pod.energy())]));
        }

        // left
        let end = pos.min(target);
        let mut first = true;
        moves.extend((0..end + 2).rev().scan(basecost, |inc, offset| {
            let mask2 = 1u32 << (6 - offset);
            let c = if first {
                let r = *inc + 2;
                *inc += COSTS[offset];
                first = false;
                r
            } else {
                *inc += COSTS[offset];
                *inc
            };
            (mask2 & hallway == 0).then(|| (mask, mask2, c * pod.energy()))
        }));

        // right
        let start = pos.max(target);
        let mut first = true;
        moves.extend((start + 2..7).scan(basecost, |inc, offset| {
            let mask2 = 1u32 << (6 - offset);
            let c = if first {
                let r = *inc + 2;
                *inc += COSTS[offset];
                first = false;
                r
            } else {
                *inc += COSTS[offset];
                *inc
            };
            (mask2 & hallway == 0).then(|| (mask, mask2, c * pod.energy()))
        }));

        (!moves.is_empty()).then(|| (pod, moves))
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
            if let Some((pod, futures)) = burrow.moves(t) {
                for future in futures {
                    let mut burrow = burrow;
                    let cost = cost + future.2;
                    if cost < min {
                        burrow.commit(pod, future);
                        if burrow.is_empty() {
                            min = cost;
                        } else {
                            stack.push((burrow, cost));
                        }
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
    let r = entry_energy(2) + exit_energy + transit_energy;
    assert_eq!(r, 18195);
    r
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
    let r = entry_energy(4) + exit_energy + transit_energy;
    assert_eq!(r, 50265);
    r
}
