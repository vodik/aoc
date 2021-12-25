use std::fmt::Write;

const WIDTH: usize = 139;
const HEIGHT: usize = 137;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Cucumber {
    Right,
    Down,
    Empty,
}

impl Cucumber {
    fn is_empty(&self) -> bool {
        matches!(self, Cucumber::Empty)
    }
}

fn below(point: usize) -> usize {
    point
        .checked_add(WIDTH)
        .filter(|&p| p < WIDTH * HEIGHT)
        .unwrap_or(point % WIDTH)
}

fn right(point: usize) -> usize {
    point
        .checked_add(1)
        .filter(|p| p % WIDTH != 0)
        .unwrap_or(point / WIDTH * WIDTH)
}

pub fn parse_input(input: &str) -> Vec<Cucumber> {
    input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|byte| match byte {
                b'>' => Cucumber::Right,
                b'v' => Cucumber::Down,
                b'.' => Cucumber::Empty,
                _ => unreachable!(),
            })
        })
        .collect()
}

fn step(board: &[Cucumber]) -> (Vec<Cucumber>, usize) {
    let mut newboard = vec![Cucumber::Empty; board.len()];
    let mut downs = Vec::with_capacity(4800);
    let mut moves = 0;

    for (pos, cell) in board.iter().enumerate() {
        match cell {
            Cucumber::Right => {
                let target = right(pos);
                if board[target].is_empty() {
                    moves += 1;
                    newboard[target] = Cucumber::Right;
                } else {
                    newboard[pos] = Cucumber::Right;
                }
            }
            Cucumber::Down => {
                downs.push(pos);
            }
            Cucumber::Empty => {}
        };
    }

    for pos in downs {
        let target = below(pos);
        let promote = match board[target] {
            Cucumber::Down => false,
            _ => newboard[target].is_empty(),
        };

        if promote {
            moves += 1;
            newboard[target] = Cucumber::Down;
        } else {
            newboard[pos] = Cucumber::Down;
        }
    }

    (newboard, moves)
}

fn dump(board: &[Cucumber]) -> String {
    let mut output = String::with_capacity(board.len());

    for (idx, p) in board.iter().enumerate() {
        let c = match p {
            Cucumber::Right => '>',
            Cucumber::Down => 'v',
            Cucumber::Empty => '.',
        };
        write!(&mut output, "{}", c).unwrap();

        if idx % WIDTH == WIDTH - 1 {
            writeln!(&mut output).unwrap();
        }
    }
    output
}

pub fn part1(input: &[Cucumber]) -> usize {
    let board = input.to_vec();
    (1..)
        .scan(board, |board, idx| {
            let (newboard, count) = step(board);
            *board = newboard;
            (count > 0).then(|| idx)
        })
        .last()
        .unwrap() + 1
}

pub fn part2(input: &[Cucumber]) -> usize {
    0
}
