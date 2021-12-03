#[derive(Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Op(Direction, u32);

pub fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let (direction, step) = line.split_once(' ').unwrap();
            Op(
                match direction {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => unreachable!(),
                },
                step.parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[Op]) -> u32 {
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    for Op(direction, step) in input {
        match direction {
            Direction::Forward => {
                horizontal += step;
            }
            Direction::Up => {
                depth = depth.saturating_sub(*step);
            }
            Direction::Down => {
                depth += step;
            }
        }
    }

    horizontal * depth
}

pub fn part2(input: &[Op]) -> u32 {
    let mut aim: i32 = 0;
    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    for Op(direction, step) in input {
        match direction {
            Direction::Forward => {
                horizontal += step;

                let delta = aim * i32::try_from(*step).unwrap();
                if delta.is_negative() {
                    depth = depth.saturating_sub(delta.wrapping_abs() as u32);
                } else {
                    depth += delta as u32;
                }
            }
            Direction::Up => {
                aim -= i32::try_from(*step).unwrap();
            }
            Direction::Down => {
                aim += i32::try_from(*step).unwrap();
            }
        }
    }

    horizontal * depth
}
