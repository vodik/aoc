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

#[derive(Debug, Default, Clone, Copy)]
struct BitBoard {
    amber: u8,
    bronze: u8,
    copper: u8,
    desert: u8,
}

impl BitBoard {
    fn new(amber: u8, bronze: u8, copper: u8, desert: u8) -> Self {
        Self {
            amber,
            bronze,
            copper,
            desert,
        }
    }

    fn flatten(&self) -> u8 {
        self.amber | self.bronze | self.copper | self.desert
    }

    fn is_empty(&self) -> bool {
        self.flatten() == 0
    }

    fn extract(&self) -> Option<(Amphipod, u8)> {
        let bits = self.flatten();
        if bits == 0 {
            return None;
        }

        let mut mask = None;
        for idx in 0..4 {
            let m = 1 << idx;
            if bits & m == m {
                mask = Some(idx);
                break;
            }
        }

        let mask = 1 << mask?;
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

        Some((pod, mask))
    }
}

#[derive(Debug, Clone, Copy)]
enum Room {
    Room {
        slots: [Option<Amphipod>; 4],
        pos: usize,
        len: usize,
    },
    Empty,
}

impl Room {
    fn extract(&self) -> Option<&Amphipod> {
        match self {
            Room::Room {
                slots: room, pos, ..
            } => room[*pos].as_ref(),
            Room::Empty => None,
        }
    }

    fn take(&mut self) {
        match self {
            Room::Room {
                slots: room,
                pos,
                len,
            } => {
                room[*pos].take();
                *pos += 1;
                if *pos == *len {
                    *self = Room::Empty;
                }
            }
            Room::Empty => {}
        }
    }

    fn is_empty(&self) -> bool {
        self.extract().is_none()
    }
}

#[derive(Debug, Clone, Copy)]
struct Burrow {
    rooms: [Room; 4],
    hallway: BitBoard,
}

const HALLWAY_MASK: u32 = 0b1111111;

impl Burrow {
    fn new(rooms: &[&[Amphipod]; 4]) -> Self {
        let mut r: Vec<Room> = Vec::new();

        for (room, members) in rooms.iter().enumerate() {
            let mut pods = Vec::new();
            // let mut amber = 0;
            // let mut bronze = 0;
            // let mut copper = 0;
            // let mut desert = 0;

            for (idx, pod) in members.iter().enumerate() {
                pods.push(Some(*pod));
                // let mask = 1 << (idx) as u8;
                // match pod {
                //     Amphipod::Amber => amber |= mask,
                //     Amphipod::Bronze => bronze |= mask,
                //     Amphipod::Copper => copper |= mask,
                //     Amphipod::Desert => desert |= mask,
                // };
            }
            let l = pods.len();
            while pods.len() < 4 {
                pods.push(None);
            }
            // r.push(BitBoard::new(amber, bronze, copper, desert));
            r.push(Room::Room {
                slots: pods.try_into().unwrap(),
                pos: 0,
                len: l,
            });
        }

        Self {
            rooms: r.try_into().unwrap(),
            hallway: BitBoard::default(),
        }
    }

    fn has_path(&self, start: usize, dist: usize) -> bool {
        if dist == 0 {
            true
        } else {
            let mask = ((2 << (dist - 1)) - 1) << (5 - dist - start);
            self.hallway.flatten() & mask == 0
        }
    }

    // fn commit(&mut self, pod: Amphipod, target: usize, mask1: u8, mask2: u8) {
    fn commit(&mut self, pod: Amphipod, target: usize, mask2: u8) {
        match pod {
            Amphipod::Amber => {
                self.rooms[target].take();
                // self.rooms[target].amber &= !mask1;
                self.hallway.amber |= mask2;
            }
            Amphipod::Bronze => {
                self.rooms[target].take();
                // self.rooms[target].bronze &= !mask1;
                self.hallway.bronze |= mask2;
            }
            Amphipod::Copper => {
                self.rooms[target].take();
                // self.rooms[target].copper &= !mask1;
                self.hallway.copper |= mask2;
            }
            Amphipod::Desert => {
                // self.rooms[target].desert &= !mask1;
                self.rooms[target].take();
                self.hallway.desert |= mask2;
            }
        }

        if self.rooms[0].is_empty() {
            self.hallway.amber = 0;
        }
        if self.rooms[1].is_empty() {
            self.hallway.bronze = 0;
        }
        if self.rooms[2].is_empty() {
            self.hallway.copper = 0;
        }
        if self.rooms[3].is_empty() {
            self.hallway.desert = 0;
        }
    }

    // fn moves(&self, pos: usize) -> Option<(Amphipod, usize, u8, Vec<(u8, usize)>)> {
    fn moves(&self, pos: usize) -> Option<(Amphipod, usize, Vec<(u8, usize)>)> {
        // let (pod, mask) = self.rooms[pos].extract()?;
        let pod = self.rooms[pos].extract()?;
        // let pod = self.extract(pos)?;
        // dbg!((pod, mask));

        // FROM HERE
        let energy = pod.energy();
        let target = match pod {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        };

        const COSTS: [usize; 7] = [2, 2, 4, 4, 4, 2, 2];
        let hallway = self.hallway.flatten();
        let mut moves = Vec::with_capacity(7);

        let (start, end) = if pos < target {
            (pos, target)
        } else {
            (target, pos)
        };

        let dist = end - start;
        let basecost = dist * 2;
        let has_path = self.has_path(start, dist);

        if has_path && self.rooms[target].is_empty() {
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

        // (!moves.is_empty()).then(|| (pod, pos, mask, moves))
Some((*pod, pos, moves))
        // (!moves.is_empty()).then(|| (*pod, pos, moves))
    }
}

fn solve(rooms: &[&[Amphipod]; 4]) -> usize {
    let mut stack = vec![(Burrow::new(rooms), 0)];
    let mut min = usize::MAX;

    while let Some((burrow, cost)) = stack.pop() {
        for t in (0..4).rev() {
            if let Some((pod, target, futures)) = burrow.moves(t) {
                // if let Some((pod, target, mask, futures)) = burrow.moves(t) {
                for future in futures {
                    let mut burrow = burrow;
                    let cost = cost + future.1;
                    if cost < min {
                        // burrow.commit(pod, target, mask, future.0);
                        burrow.commit(pod, target, future.0);
                        if burrow.hallway.is_empty() {
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
