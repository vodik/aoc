use std::collections::HashMap;

const WIDTH: usize = 100;
const LIMIT: usize = 1_000_000_000;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Round,
    Cube,
}

pub fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|c| match c {
                b'.' => Tile::Empty,
                b'O' => Tile::Round,
                b'#' => Tile::Cube,
                _ => unreachable!(),
            })
        })
        .collect()
}

fn tilt_up(map: &[Tile]) -> Vec<Tile> {
    let mut new_map = vec![Tile::Empty; map.len()];

    for x in 0..WIDTH {
        let mut rocks = 0;

        let mut y_cursor = 0;
        for y in 0..WIDTH {
            match map[y * WIDTH + x] {
                Tile::Empty => {}
                Tile::Round => rocks += 1,
                Tile::Cube => {
                    (0..rocks).for_each(|i| {
                        new_map[(y_cursor + i) * WIDTH + x] = Tile::Round;
                    });
                    new_map[y * WIDTH + x] = Tile::Cube;
                    rocks = 0;
                    y_cursor = y + 1;
                }
            }
        }

        (0..rocks).for_each(|i| {
            new_map[(y_cursor + i) * WIDTH + x] = Tile::Round;
        });
    }

    new_map
}

fn tilt_down(map: &[Tile]) -> Vec<Tile> {
    let mut new_map = vec![Tile::Empty; map.len()];

    for x in 0..WIDTH {
        let mut rocks = 0;

        let mut y_cursor = WIDTH - 1;
        for y in (0..WIDTH).rev() {
            match map[y * WIDTH + x] {
                Tile::Empty => {},
                Tile::Round => rocks += 1,
                Tile::Cube => {
                    (0..rocks).for_each(|i| {
                        new_map[(y_cursor - i) * WIDTH + x] = Tile::Round;
                    });
                    new_map[y * WIDTH + x] = Tile::Cube;
                    rocks = 0;
                    y_cursor = y - 1;
                }
            }
        }

        (0..rocks).for_each(|i| {
            new_map[(y_cursor - i) * WIDTH + x] = Tile::Round;
        });
    }

    new_map
}

fn tilt_left(map: &[Tile]) -> Vec<Tile> {
    let mut new_map = vec![Tile::Empty; map.len()];

    for y in 0..WIDTH {
        let mut rocks = 0;

        let mut x_cursor = 0;
        for x in 0..WIDTH {
            match map[y * WIDTH + x] {
                Tile::Empty => {},
                Tile::Round => rocks += 1,
                Tile::Cube => {
                    (0..rocks).for_each(|i| {
                        new_map[y * WIDTH + x_cursor + i] = Tile::Round;
                    });
                    new_map[y * WIDTH + x] = Tile::Cube;
                    rocks = 0;
                    x_cursor = x + 1;
                }
            }
        }

        (0..rocks).for_each(|i| {
            new_map[y * WIDTH + x_cursor + i] = Tile::Round;
        });
    }

    new_map
}

fn tilt_right(map: &[Tile]) -> Vec<Tile> {
    let mut new_map = vec![Tile::Empty; map.len()];

    for y in 0..WIDTH {
        let mut rocks = 0;

        let mut x_cursor = WIDTH - 1;
        for x in (0..WIDTH).rev() {
            match map[y * WIDTH + x] {
                Tile::Empty => {},
                Tile::Round => rocks += 1,
                Tile::Cube => {
                    (0..rocks).for_each(|i| {
                        new_map[y * WIDTH + x_cursor - i] = Tile::Round;
                    });
                    new_map[y * WIDTH + x] = Tile::Cube;
                    rocks = 0;
                    x_cursor = x - 1;
                }
            }
        }

        (0..rocks).for_each(|i| {
            new_map[y * WIDTH + x_cursor - i] = Tile::Round;
        });
    }

    new_map
}

fn calculate_load(new_map: &[Tile]) -> usize {
    new_map
        .chunks(WIDTH)
        .zip((1..WIDTH + 1).rev())
        .map(|(chunk, distance)| {
            chunk.iter().filter(|&&tile| tile == Tile::Round).count() * distance
        })
        .sum()
}

pub fn part1(map: &[Tile]) -> usize {
    calculate_load(&tilt_up(map))
}

pub fn part2(map: &[Tile]) -> usize {
    let mut new_map = map.to_vec();

    let mut seen = HashMap::new();

    for i in 1.. {
        new_map = tilt_up(&new_map);
        new_map = tilt_left(&new_map);
        new_map = tilt_down(&new_map);
        new_map = tilt_right(&new_map);

        if let Some(prev_i) = seen.insert(new_map.clone(), i) {
            let period = i - prev_i;
            let remainder = (LIMIT - i) % period;
            for _ in 0..remainder {
                new_map = tilt_up(&new_map);
                new_map = tilt_left(&new_map);
                new_map = tilt_down(&new_map);
                new_map = tilt_right(&new_map);
            }
            break;
        }
    }

    calculate_load(&new_map)
}
