#[derive(Debug, Copy, Clone)]
pub enum Op {
    Left(i32),
    Right(i32),
}

impl Op {
    fn step(&self) -> i32 {
        match *self {
            Op::Left(step) => step,
            Op::Right(step) => step,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let (dir, step) = line.split_at(1);
            let step = step.parse::<i32>().unwrap();
            match dir {
                "L" => Op::Left(-step),
                "R" => Op::Right(step),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(ops: &[Op]) -> u32 {
    ops.iter()
        .scan(50, |pos, op| {
            *pos = (*pos + op.step()).rem_euclid(100);
            Some((*pos == 0) as u32)
        })
        .sum()
}

pub fn part2(ops: &[Op]) -> u32 {
    ops.iter()
        .scan(50i32, |pos, op| {
            let residue = match op {
                Op::Left(_) => *pos,
                Op::Right(_) => (-*pos).rem_euclid(100),
            };

            *pos = (*pos + op.step()).rem_euclid(100);

            let first_zero_step = ((residue + 99) % 100) + 1;
            let overshoot = op.step().abs() - first_zero_step;
            let mask = (overshoot >= 0) as u32;
            Some(mask * (1 + overshoot as u32 / 100))
        })
        .sum()
}
