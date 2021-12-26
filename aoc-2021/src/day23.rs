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

    fn has_path(&self, start: usize, dist: usize) -> bool {
        if dist == 0 {
            true
        } else {
            let mask = ((2 << (dist - 1)) - 1) << (5 - dist - start);
            self.hallway() & mask == 0
        }
    }

    fn commit(&mut self, pod: Amphipod, mask1: u32, mask2: u32) {
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

    fn moves(&self, pos: usize) -> Option<(Amphipod, u32, Vec<(u32, usize)>)> {
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

        let energy = pod.energy();
        let target = match pod {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        };

        const COSTS: [usize; 7] = [2, 2, 4, 4, 4, 2, 2];
        let hallway = self.hallway();
        let mut moves = Vec::with_capacity(7);

        let (start, end) = if pos < target {
            (pos, target)
        } else {
            (target, pos)
        };

        let dist = end - start;
        let basecost = dist * 2;
        let has_path = self.has_path(start, dist);

        if has_path && self.is_room_empty(target) {
            moves.push((0, basecost * energy));
        } else {
            if has_path || pos < target {
                moves.extend(
                    (0..start + 2)
                        .rev()
                        .map(|offset| (1 << (6 - offset), COSTS[offset]))
                        .scan(0, |acc, (mask2, weight)| {
                            let cost = if *acc == 0 {
                                *acc += weight;
                                2
                            } else {
                                *acc += weight;
                                *acc
                            };

                            (mask2 & hallway == 0).then(|| (mask2, (basecost + cost) * energy))
                        }),
                );
            }

            if has_path || target < pos {
                moves.extend(
                    (end + 2..7)
                        .map(|offset| (1 << (6 - offset), COSTS[offset]))
                        .scan(0, |acc, (mask2, weight)| {
                            let cost = if *acc == 0 {
                                *acc += weight;
                                2
                            } else {
                                *acc += weight;
                                *acc
                            };

                            (mask2 & hallway == 0).then(|| (mask2, (basecost + cost) * energy))
                        }),
                );
            }
        }

        (!moves.is_empty()).then(|| (pod, mask, moves))
    }
}

fn solve(rooms: &[&[Amphipod]; 4]) -> usize {
    let mut stack = vec![(Burrow::new(rooms), 0)];
    let mut min = usize::MAX;

    while let Some((burrow, cost)) = stack.pop() {
        for t in (0..4).rev() {
            if let Some((pod, mask, futures)) = burrow.moves(t) {
                for future in futures {
                    let mut burrow = burrow;
                    let cost = cost + future.1;
                    if cost < min {
                        burrow.commit(pod, mask, future.0);
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
