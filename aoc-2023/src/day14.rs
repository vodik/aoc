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

fn tilt_north(map: &mut [Tile]) {
    for x in 0..WIDTH {
        let mut stop = 0;
        for y in 0..WIDTH {
            let c = map[y * WIDTH + x];
            match c {
                Tile::Round => {
                    map.swap(y * WIDTH + x, stop * WIDTH + x);
                    stop += 1;
                }
                Tile::Cube => {
                    stop = y + 1;
                }
                _ => {}
            }
        }
    }
}

fn tilt_west(map: &mut [Tile]) {
    for y in 0..WIDTH {
        let mut stop = 0;
        for x in 0..WIDTH {
            let c = map[y * WIDTH + x];
            match c {
                Tile::Round => {
                    map.swap(y * WIDTH + x, y * WIDTH + stop);
                    stop += 1;
                }
                Tile::Cube => {
                    stop = x + 1;
                }
                _ => {}
            }
        }
    }
}

fn tilt_south(map: &mut [Tile]) {
    for x in 0..WIDTH {
        let mut stop = WIDTH - 1;
        for y in (0..WIDTH).rev() {
            let c = map[y * WIDTH + x];
            match c {
                Tile::Round => {
                    map.swap(y * WIDTH + x, stop * WIDTH + x);
                    stop = stop.wrapping_sub(1);
                }
                Tile::Cube => {
                    stop = y.wrapping_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn tilt_east(map: &mut [Tile]) {
    for y in 0..WIDTH {
        let mut stop = WIDTH - 1;
        for x in (0..WIDTH).rev() {
            let c = map[y * WIDTH + x];
            match c {
                Tile::Round => {
                    map.swap(y * WIDTH + x, y * WIDTH + stop);
                    stop = stop.wrapping_sub(1);
                }
                Tile::Cube => {
                    stop = x.wrapping_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn calculate_load(map: &[Tile]) -> usize {
    map.chunks(WIDTH)
        .zip((1..WIDTH + 1).rev())
        .map(|(chunk, distance)| {
            chunk.iter().filter(|&&tile| tile == Tile::Round).count() * distance
        })
        .sum()
}

pub fn part1(map: &[Tile]) -> usize {
    let mut map = map.to_vec();
    tilt_north(&mut map);
    calculate_load(&map)
}

pub fn part2(map: &[Tile]) -> usize {
    let mut map = map.to_vec();
    let mut seen = HashMap::new();

    for i in 1.. {
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);

        if let Some(prev_i) = seen.insert(map.clone(), i) {
            let period = i - prev_i;
            let remainder = (LIMIT - i) % period;
            for _ in 0..remainder {
                tilt_north(&mut map);
                tilt_west(&mut map);
                tilt_south(&mut map);
                tilt_east(&mut map);
            }
            break;
        }
    }

    calculate_load(&map)
}
