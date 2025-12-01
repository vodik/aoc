#[derive(Debug, Copy, Clone)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Op {
    dir: Dir,
    step: i32,
    step_size: i32,
}

impl Op {
    pub fn new(dir: Dir, step_size: u32) -> Self {
        let step_size = step_size as i32;
        Self {
            dir,
            step: match dir {
                Dir::Left => -step_size,
                Dir::Right => step_size,
            },
            step_size,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let (dir, step) = line.split_at(1);
            let step_size = step.parse::<u32>().unwrap();
            match dir {
                "L" => Op::new(Dir::Left, step_size),
                "R" => Op::new(Dir::Right, step_size),
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
        .scan(50, |pos, op| {
            let residue = match op.dir {
                Dir::Left => *pos,
                Dir::Right => 100 - *pos,
            };

            *pos = (*pos + op.step).rem_euclid(100);

            let first_zero_step = ((residue + 99) % 100) + 1;
            let overshoot = op.step_size - first_zero_step;
            let mask = (overshoot >= 0) as u32;
            Some(mask * (1 + overshoot as u32 / 100))
        })
        .sum()
}
