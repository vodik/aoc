#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Op {
    dir: Dir,
    step: i32,
}

impl Op {
    pub fn new(dir: Dir, step: i32) -> Self {
        Self { dir, step }
    }
}

pub fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let (dir, step) = line.split_at(1);
            let step = step.parse::<i32>().unwrap();
            match dir {
                "L" => Op::new(Dir::Left, -step),
                "R" => Op::new(Dir::Right, step),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(ops: &[Op]) -> u32 {
    ops.iter()
        .scan(50, |pos, op| {
            *pos = (*pos + op.step).rem_euclid(100);
            Some((*pos == 0) as u32)
        })
        .sum()
}

pub fn part2(ops: &[Op]) -> u32 {
    ops.iter()
        .scan(50i32, |pos, op| {
            let residue = match op.dir {
                Dir::Left => *pos,
                Dir::Right => (-*pos).rem_euclid(100),
            };

            *pos = (*pos + op.step).rem_euclid(100);

            let first_zero_step = ((residue + 99) % 100) + 1;
            let overshoot = op.step.abs() - first_zero_step;
            let mask = (overshoot >= 0) as u32;
            Some(mask * (1 + overshoot as u32 / 100))
        })
        .sum()
}
