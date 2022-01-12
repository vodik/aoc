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

fn exit_energy<const N: usize>(rooms: &[&[Amphipod; N]; 4]) -> usize {
    let exit_energy: usize = rooms
        .iter()
        .flat_map(|room| {
            room.iter()
                .enumerate()
                .map(|(idx, pod)| (idx + 1) * pod.energy())
        })
        .sum();
    exit_energy
}

#[derive(Debug, Default, Clone, Copy)]
struct BitBoard {
    amber: u8,
    bronze: u8,
    copper: u8,
    desert: u8,
}

impl BitBoard {
    fn flatten(&self) -> u8 {
        self.amber | self.bronze | self.copper | self.desert
    }

    fn is_empty(&self) -> bool {
        self.flatten() == 0
    }
}

#[derive(Debug, Clone, Copy)]
enum Room<const N: usize> {
    Room {
        slots: [Option<Amphipod>; N],
        pos: usize,
    },
    Empty,
}

impl<const N: usize> Room<N> {
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
            Room::Room { slots: room, pos } => {
                room[*pos].take();
                *pos += 1;
                if *pos == N {
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
struct Burrow<const N: usize> {
    rooms: [Room<N>; 4],
    hallway: BitBoard,
}

impl<const N: usize> Burrow<N> {
    fn new(rooms: &[&[Amphipod; N]; 4]) -> Self {
        let mut r: Vec<Room<N>> = Vec::new();

        for &members in rooms {
            let pods: Vec<Option<_>> = members.iter().copied().map(Some).collect();

            r.push(Room::Room {
                slots: pods.try_into().unwrap(),
                pos: 0,
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

    fn commit(&mut self, pod: Amphipod, target: usize, mask: u8) {
        match pod {
            Amphipod::Amber => {
                self.rooms[target].take();
                self.hallway.amber |= mask;
            }
            Amphipod::Bronze => {
                self.rooms[target].take();
                self.hallway.bronze |= mask;
            }
            Amphipod::Copper => {
                self.rooms[target].take();
                self.hallway.copper |= mask;
            }
            Amphipod::Desert => {
                self.rooms[target].take();
                self.hallway.desert |= mask;
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

    fn moves(&self, pos: usize) -> Option<(Amphipod, usize, Vec<(u8, usize)>)> {
        let pod = self.rooms[pos].extract()?;

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

        Some((*pod, pos, moves))
    }
}

fn solve<const N: usize>(rooms: &[&[Amphipod; N]; 4]) -> usize {
    let mut stack = vec![(Burrow::<N>::new(rooms), 0)];
    let mut min = usize::MAX;

    while let Some((burrow, cost)) = stack.pop() {
        for t in (0..4).rev() {
            if let Some((pod, target, futures)) = burrow.moves(t) {
                for future in futures {
                    let mut burrow = burrow;
                    let cost = cost + future.1;
                    if cost < min {
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
    let rooms: [&[Amphipod; 2]; 4] = [
        &[Amphipod::Amber, Amphipod::Desert],
        &[Amphipod::Copper, Amphipod::Desert],
        &[Amphipod::Bronze, Amphipod::Bronze],
        &[Amphipod::Amber, Amphipod::Copper],
    ];

    let energy = entry_energy(2) + exit_energy(&rooms) + solve(&rooms);
    assert_eq!(energy, 18195);
    energy
}

pub fn part2(input: &()) -> usize {
    let rooms: [&[Amphipod; 4]; 4] = [
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

    let energy = entry_energy(4) + exit_energy(&rooms) + solve(&rooms);
    assert_eq!(energy, 50265);
    energy
}
