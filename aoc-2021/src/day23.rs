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
    fn moves(&self, pos: usize) -> Option<Vec<(usize, Amphipod, u32, u32, usize)>> {
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

        if !self.has_path(pos, target) {
            return None;
        }

        let basecost = abs(pos, target) * 2;
        if self.is_room_empty(target) {
            return Some(vec![(pos, pod, mask, 0, basecost * pod.energy())]);
        }

        let hallway = self.hallway();
        let mut moves = Vec::with_capacity(6);

        const COSTS: [usize; 7] = [2, 4, 4, 4, 4, 4, 2];

        // left
        let start = match pos.min(target) {
            0 => 5,
            1 => 4,
            2 => 3,
            3 => 2,
            _ => unreachable!(),
        };

        moves.extend((start..7).scan(basecost + 2, |inc, offset| {
            let mask2 = 1u32 << offset;
            let &cost = COSTS.get(offset + 1).unwrap_or(&2);
            let c = *inc;
            *inc += cost;
            (mask2 & hallway == 0).then(|| (pos, pod, mask, mask2, c * pod.energy()))
        }));

        // right
        let end = match pos.max(target) {
            0 => 4,
            1 => 3,
            2 => 2,
            3 => 1,
            _ => unreachable!(),
        };

        moves.extend((0..end + 1).rev().scan(basecost + 2, |inc, offset| {
            let mask2 = 1u32 << offset;
            let &cost = COSTS.get(offset + 1).unwrap_or(&2);
            let c = *inc;
            *inc += cost;
            (mask2 & hallway == 0).then(|| (pos, pod, mask, mask2, c * pod.energy()))
        }));

        Some(moves)
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
            if let Some(futures) = burrow.moves(t) {
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
    entry_energy(2) + exit_energy + transit_energy
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
    entry_energy(2) + exit_energy + transit_energy
}
