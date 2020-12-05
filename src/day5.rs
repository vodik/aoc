use std::collections::BinaryHeap;
use std::ops::Sub;

#[derive(Debug)]
enum Op {
    Top,
    Bottom,
}

#[derive(Debug)]
struct Seat {
    row: Vec<Op>,
    column: Vec<Op>,
}

#[derive(Debug)]
enum Partition {
    Range(u32, u32),
    Single(u32),
}

fn distance<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl Partition {
    fn new(start: u32, end: u32) -> Self {
        if distance(start, end) == 0 {
            Partition::Single(start)
        } else {
            Partition::Range(start, end)
        }
    }

    fn split(self, op: &Op) -> Partition {
        match self {
            Partition::Range(start, end) => {
                let width = end - start;
                let midpoint = start + width / 2;
                match op {
                    Op::Top => Partition::new(start, midpoint),
                    Op::Bottom => Partition::new(midpoint + 1, end),
                }
            }
            _ => panic!(),
        }
    }

    fn unwrap(self) -> u32 {
        match self {
            Partition::Single(value) => value,
            _ => panic!(),
        }
    }
}

impl Seat {
    fn seat_id(&self) -> u32 {
        let row = self
            .row
            .iter()
            .fold(Partition::new(0, 127), |partition, op| partition.split(op))
            .unwrap();

        let column = self
            .column
            .iter()
            .fold(Partition::new(0, 7), |partition, op| partition.split(op))
            .unwrap();

        row * 8 + column
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| Seat {
            row: line[0..7]
                .chars()
                .map(|c| match c {
                    'F' => Op::Top,
                    'B' => Op::Bottom,
                    _ => unreachable!(),
                })
                .collect(),
            column: line[7..]
                .chars()
                .map(|c| match c {
                    'L' => Op::Top,
                    'R' => Op::Bottom,
                    _ => unreachable!(),
                })
                .collect(),
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(data: &[Seat]) -> Option<u32> {
    data.iter().map(|seat| seat.seat_id()).max()
}

#[aoc(day5, part2)]
fn part2(data: &[Seat]) -> Option<u32> {
    let seats = data
        .iter()
        .map(|seat| seat.seat_id())
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec();

    seats.windows(2).find_map(|data| {
        if distance(data[0], data[1]) == 2 {
            Some(data[0] as u32 + 1)
        } else {
            None
        }
    })
}
