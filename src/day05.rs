use std::collections::BinaryHeap;

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

impl Seat {
    fn seat_id(&self) -> u32 {
        let row = bsp(&self.row);
        let column = bsp(&self.column);

        row * 8 + column
    }
}

fn bsp(ops: &[Op]) -> u32 {
    ops.iter().fold(0, |acc, op| {
        acc << 1
            | match op {
                Op::Top => 0,
                Op::Bottom => 1,
            }
    })
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
        if data[0] + 1 != data[1] {
            Some(data[0] as u32 + 1)
        } else {
            None
        }
    })
}
