const WIDTH: usize = 140;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Start,
    Ground,
    Vertical,
    Horizonal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
}

pub fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|c| match c {
                b'S' => Tile::Start,
                b'.' => Tile::Ground,
                b'|' => Tile::Vertical,
                b'-' => Tile::Horizonal,
                b'L' => Tile::BendNE,
                b'J' => Tile::BendNW,
                b'F' => Tile::BendSE,
                b'7' => Tile::BendSW,
                _ => Tile::Ground,
            })
        })
        .collect()
}

fn up(point: usize, width: usize) -> Option<usize> {
    point.checked_sub(width)
}

fn left(point: usize, width: usize) -> Option<usize> {
    point.checked_sub(1).filter(|p| p % width != width - 1)
}

fn down(point: usize, width: usize) -> Option<usize> {
    point.checked_add(width).filter(|&p| p < width * width)
}

fn right(point: usize, width: usize) -> Option<usize> {
    point.checked_add(1).filter(|p| p % width != 0)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn try_step(map: &[Tile], (step, position): (Direction, usize)) -> Option<(Direction, usize)> {
    match step {
        Direction::North => {
            let next_pos = up(position, WIDTH)?;
            let next_step = match map[next_pos] {
                Tile::Vertical => Some(Direction::North),
                Tile::BendSE => Some(Direction::East),
                Tile::BendSW => Some(Direction::West),
                _ => None,
            };
            next_step.map(|step| (step, next_pos))
        }
        Direction::East => {
            let next_pos = right(position, WIDTH)?;
            let next_step = match map[next_pos] {
                Tile::Horizonal => Some(Direction::East),
                Tile::BendNW => Some(Direction::North),
                Tile::BendSW => Some(Direction::South),
                _ => None,
            };
            next_step.map(|step| (step, next_pos))
        }
        Direction::South => {
            let next_pos = down(position, WIDTH)?;
            let next_step = match map[next_pos] {
                Tile::Vertical => Some(Direction::South),
                Tile::BendNE => Some(Direction::East),
                Tile::BendNW => Some(Direction::West),
                _ => None,
            };
            next_step.map(|step| (step, next_pos))
        }
        Direction::West => {
            let next_pos = left(position, WIDTH)?;
            let next_step = match map[next_pos] {
                Tile::Horizonal => Some(Direction::West),
                Tile::BendNE => Some(Direction::North),
                Tile::BendSE => Some(Direction::South),
                _ => None,
            };
            next_step.map(|step| (step, next_pos))
        }
    }
}

pub fn part1(map: &[Tile]) -> usize {
    let start = map.iter().position(|&tile| tile == Tile::Start).unwrap();
    let mut cursors: Vec<_> = [
        try_step(map, (Direction::North, start)),
        try_step(map, (Direction::East, start)),
        try_step(map, (Direction::South, start)),
        try_step(map, (Direction::West, start)),
    ]
    .into_iter()
    .flatten()
    .collect();

    let mut steps = 1;
    while cursors[0].1 != cursors[1].1 {
        for cursor in cursors.iter_mut() {
            *cursor = try_step(map, *cursor).unwrap();
        }
        steps += 1;
    }
    steps
}

pub fn part2(map: &[Tile]) -> usize {
    let mut border = vec![false; map.len()];

    let start = map.iter().position(|&tile| tile == Tile::Start).unwrap();
    let mut cursors: Vec<_> = [
        try_step(map, (Direction::North, start)),
        try_step(map, (Direction::East, start)),
        try_step(map, (Direction::South, start)),
        try_step(map, (Direction::West, start)),
    ]
    .into_iter()
    .flatten()
    .collect();

    border[start] = true;
    for cursor in &cursors {
        border[cursor.1] = true;
    }

    while cursors[0].1 != cursors[1].1 {
        for cursor in cursors.iter_mut() {
            *cursor = try_step(map, *cursor).unwrap();
            border[cursor.1] = true;
        }
    }

    let mut counter = 0;
    for (edges, tiles) in border.chunks(WIDTH).zip(map.chunks(WIDTH)) {
        let mut inside = false;
        let mut direction = None;
        for (&edge, &tile) in edges.iter().zip(tiles.iter()) {
            if edge {
                match tile {
                    Tile::Vertical => {
                        inside ^= true;
                    }
                    Tile::Start | Tile::BendNE => {
                        inside ^= true;
                        direction = Some(Direction::North);
                    }
                    Tile::BendNW => {
                        if direction == Some(Direction::North) {
                            inside ^= true;
                        }
                        direction = None;
                    }
                    Tile::BendSE => {
                        inside ^= true;
                        direction = Some(Direction::South);
                    }
                    Tile::BendSW => {
                        if direction == Some(Direction::South) {
                            inside ^= true;
                        }
                        direction = None;
                    }
                    _ => {}
                };
            } else if inside {
                counter += 1;
            }
        }
    }
    counter
}
