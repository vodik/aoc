use crate::parsers::{grid, number};
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    data: [u8; 100],
}

impl Tile {
    fn top(&self) -> u16 {
        self.row(0)
    }

    fn right(&self) -> u16 {
        self.column(9)
    }

    fn bottom(&self) -> u16 {
        self.row(9)
    }

    fn left(&self) -> u16 {
        self.column(0)
    }

    fn row(&self, pos: usize) -> u16 {
        (0..10).fold(0, |acc, idx| {
            let idx = pos * 10 + idx;
            acc << 1 | if self.data[idx] == b'#' { 1 } else { 0 }
        })
    }

    fn column(&self, pos: usize) -> u16 {
        (0..10).fold(0, |acc, idx| {
            let idx = idx * 10 + pos;
            acc << 1 | if self.data[idx] == b'#' { 1 } else { 0 }
        })
    }

    fn edges(&self) -> [u16; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }

    fn transform(&mut self, transform: &Transform) {
        let mut new_tile = [0; 100];
        for (idx, &src) in transform.matrix().iter().enumerate() {
            new_tile[idx] = self.data[src];
        }
        self.data = new_tile;
    }
}

#[derive(Debug)]
enum Transform {
    Left,
    Right,
    FlipX,
    FlipY,
}

impl Transform {
    fn matrix(&self) -> [usize; 100] {
        match self {
            #[rustfmt::skip]
            Transform::Right => [
                90, 80, 70, 60, 50, 40, 30, 20, 10, 0,
                91, 81, 71, 61, 51, 41, 31, 21, 11, 1,
                92, 82, 72, 62, 52, 42, 32, 22, 12, 2,
                93, 83, 73, 63, 53, 43, 33, 23, 13, 3,
                94, 84, 74, 64, 54, 44, 34, 24, 14, 4,
                95, 85, 75, 65, 55, 45, 35, 25, 15, 5,
                96, 86, 76, 66, 56, 46, 36, 26, 16, 6,
                97, 87, 77, 67, 57, 47, 37, 27, 17, 7,
                98, 88, 78, 68, 58, 48, 38, 28, 18, 8,
                99, 89, 79, 69, 59, 49, 39, 29, 19, 9,
            ],
            #[rustfmt::skip]
            Transform::Left => [
                9, 19, 29, 39, 49, 59, 69, 79, 89, 99,
                8, 18, 28, 38, 48, 58, 68, 78, 88, 98,
                7, 17, 27, 37, 47, 57, 67, 77, 87, 97,
                6, 16, 26, 36, 46, 56, 66, 76, 86, 96,
                5, 15, 25, 35, 45, 55, 65, 75, 85, 95,
                4, 14, 24, 34, 44, 54, 64, 74, 84, 94,
                3, 13, 23, 33, 43, 53, 63, 73, 83, 93,
                2, 12, 22, 32, 42, 52, 62, 72, 82, 92,
                1, 11, 21, 31, 41, 51, 61, 71, 81, 91,
                0, 10, 20, 30, 40, 50, 60, 70, 80, 90,
            ],
            #[rustfmt::skip]
            Transform::FlipX => [
                90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
                80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
                70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
                60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
                50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
                40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
                10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                 0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
            ],
            #[rustfmt::skip]
            Transform::FlipY => [
                 9,  8,  7,  6,  5,  4,  3,  2,  1,  0,
                19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
                29, 28, 27, 26, 25, 24, 23, 22, 21, 20,
                39, 38, 37, 36, 35, 34, 33, 32, 31, 30,
                49, 48, 47, 46, 45, 44, 43, 42, 41, 40,
                59, 58, 57, 56, 55, 54, 53, 52, 51, 50,
                69, 68, 67, 66, 65, 64, 63, 62, 61, 60,
                79, 78, 77, 76, 75, 74, 73, 72, 71, 70,
                89, 88, 87, 86, 85, 84, 83, 82, 81, 80,
                99, 98, 97, 96, 95, 94, 93, 92, 91, 90,
            ],
        }
    }
}

fn tile(input: &str) -> IResult<&str, Tile> {
    map(
        separated_pair(
            delimited(tag("Tile "), number, tag(":")),
            tag("\n"),
            grid("#."),
        ),
        |(id, (grid, dim))| {
            assert_eq!(dim, (10, 10));
            let data = grid.try_into().unwrap();
            Tile { id, data }
        },
    )(input)
}

fn parse_tiles(input: &str) -> IResult<&str, Vec<Tile>> {
    separated_list1(tag("\n\n"), tile)(input)
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Result<Vec<Tile>, Error<String>> {
    match all_consuming(parse_tiles)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

fn reverse_edge(value: u16) -> u16 {
    // 10 bit number stored inside 16 bits
    value.reverse_bits() >> 6
}

fn edge_map(tiles: &[Tile]) -> HashMap<u16, usize> {
    let mut edges = HashMap::new();
    for tile in tiles.iter() {
        for &edge in &tile.edges() {
            *edges.entry(edge).or_default() += 1;
            *edges.entry(reverse_edge(edge)).or_default() += 1;
        }
    }
    edges
}

fn iter_corners<'a>(
    edges: &'a HashMap<u16, usize>,
    tiles: &'a [Tile],
) -> impl Iterator<Item = &'a Tile> + 'a {
    tiles
        .iter()
        .filter(move |tile| tile.edges().iter().map(|edge| edges[edge]).sum::<usize>() == 6)
}

#[aoc(day20, part1)]
fn part1(tiles: &[ Tile]) -> u64 {
    let edges = edge_map(tiles);
    iter_corners(&edges, tiles).fold(1, |acc, tile| acc * tile.id as u64)
}

#[aoc(day20, part2)]
fn part2(tiles: &[Tile]) -> usize {
    let width = (tiles.len() as f32).sqrt() as usize;

    let mut map = Vec::with_capacity(tiles.len());

    let edges = edge_map(tiles);
    let start_tile = iter_corners(&edges, tiles)
        .next()
        .expect("No corner tiles where found")
        .clone();

    let transforms = match (
        edges[&start_tile.top()],
        edges[&start_tile.left()],
        edges[&start_tile.bottom()],
        edges[&start_tile.right()],
    ) {
        (2, 2, 1, 1) => vec![Transform::FlipX, Transform::FlipY],
        (1, 2, 2, 1) => vec![Transform::FlipY],
        (1, 1, 2, 2) => vec![],
        (2, 1, 1, 2) => vec![Transform::FlipX],
        _ => panic!("Starting tile doesn't appear to be a corner tile"),
    };

    map.push(
        transforms
            .into_iter()
            .fold(start_tile, |mut tile, transform| {
                tile.transform(&transform);
                tile
            }),
    );

    for step in 1..tiles.len() {
        map.push(
            if step % width == 0 {
                let cursor = &map[step - width];
                let edge = cursor.bottom();
                let edge_alt = reverse_edge(edge);
                tiles
                    .iter()
                    .filter(|tile| tile.id != cursor.id)
                    .find_map(|tile| {
                        if edge == tile.top() {
                            Some((tile, vec![]))
                        } else if edge_alt == tile.top() {
                            Some((tile, vec![Transform::FlipY]))
                        } else if edge == tile.left() {
                            Some((tile, vec![Transform::Right, Transform::FlipY]))
                        } else if edge_alt == tile.left() {
                            Some((tile, vec![Transform::Right]))
                        } else if edge == tile.bottom() {
                            Some((tile, vec![Transform::FlipX]))
                        } else if edge_alt == tile.bottom() {
                            Some((tile, vec![Transform::FlipX, Transform::FlipY]))
                        } else if edge == tile.right() {
                            Some((tile, vec![Transform::Left]))
                        } else if edge_alt == tile.right() {
                            Some((tile, vec![Transform::Left, Transform::FlipY]))
                        } else {
                            None
                        }
                    })
            } else {
                let cursor = &map[step - 1];
                let edge = cursor.right();
                let edge_alt = reverse_edge(edge);
                tiles
                    .iter()
                    .filter(|tile| tile.id != cursor.id)
                    .find_map(|tile| {
                        if edge == tile.top() {
                            Some((tile, vec![Transform::Left, Transform::FlipX]))
                        } else if edge_alt == tile.top() {
                            Some((tile, vec![Transform::Left]))
                        } else if edge == tile.left() {
                            Some((tile, vec![]))
                        } else if edge_alt == tile.left() {
                            Some((tile, vec![Transform::FlipX]))
                        } else if edge == tile.bottom() {
                            Some((tile, vec![Transform::Right]))
                        } else if edge_alt == tile.bottom() {
                            Some((tile, vec![Transform::Right, Transform::FlipX]))
                        } else if edge == tile.right() {
                            Some((tile, vec![Transform::FlipY]))
                        } else if edge_alt == tile.right() {
                            Some((tile, vec![Transform::FlipX, Transform::FlipY]))
                        } else {
                            None
                        }
                    })
            }
            .map(|(tile, transforms)| {
                transforms
                    .into_iter()
                    .fold(tile.clone(), |mut tile, transform| {
                        tile.transform(&transform);
                        tile
                    })
            })
            .expect("No next tile was found"),
        );
    }

    let bitwidth = width * 8;
    let bitmap = map
        .chunks(width)
        .flat_map(|tiles| {
            (1..=8).map(move |pos| {
                tiles.iter().fold(0u128, |acc, tile| {
                    acc << 8 | (tile.row(pos) as u128 & 0x1fe) >> 1
                })
            })
        })
        .collect::<Vec<_>>();

    let monsters = bitmap
        .windows(MONSTER_HEIGHT)
        .flat_map(|window| {
            (0..=bitwidth - MONSTER_WIDTH).filter(move |shift| {
                MONSTER1
                    .iter()
                    .zip(window.iter())
                    .all(|(&mask, &line)| mask << shift | line == line)
                    || MONSTER1
                        .iter()
                        .rev()
                        .zip(window.iter())
                        .all(|(&mask, &line)| mask << shift | line == line)
                    || MONSTER2
                        .iter()
                        .zip(window.iter())
                        .all(|(&mask, &line)| mask << shift | line == line)
                    || MONSTER2
                        .iter()
                        .rev()
                        .zip(window.iter())
                        .all(|(&mask, &line)| mask << shift | line == line)
            })
        })
        .chain(bitmap.windows(MONSTER_WIDTH).flat_map(|window| {
            (0..=bitwidth - MONSTER_HEIGHT).filter(move |shift| {
                MONSTER3
                    .iter()
                    .rev()
                    .zip(window.iter())
                    .all(|(&mask, &line)| mask << shift | line == line)
                    || MONSTER3
                        .iter()
                        .zip(window.iter())
                        .all(|(&mask, &line)| mask << shift | line == line)
                    || MONSTER4
                        .iter()
                        .zip(window.iter())
                        .all(|(&mask, &line)| mask << shift | line == line)
                    || MONSTER4
                        .iter()
                        .rev()
                        .zip(window.iter())
                        .all(|(&mask, &line)| mask << shift | line == line)
            })
        }))
        .count();

    bitmap
        .iter()
        .map(|line| line.count_ones() as usize)
        .sum::<usize>()
        - monsters * MONSTER_BITS
}

const MONSTER_WIDTH: usize = 20;
const MONSTER_HEIGHT: usize = 3;
const MONSTER_BITS: usize = 15;

const MONSTER1: [u128; 3] = [
    0b00000000000000000010,
    0b10000110000110000111,
    0b01001001001001001000,
];

const MONSTER2: [u128; 3] = [
    0b01000000000000000000,
    0b11100001100001100001,
    0b00010010010010010010,
];

const MONSTER3: [u128; 20] = [
    0b010, 0b110, 0b010, 0b001, 0b000, 0b000, 0b001, 0b010, 0b010, 0b001, 0b000, 0b000, 0b001,
    0b010, 0b010, 0b001, 0b000, 0b000, 0b001, 0b010,
];

const MONSTER4: [u128; 20] = [
    0b010, 0b011, 0b010, 0b100, 0b000, 0b000, 0b100, 0b010, 0b010, 0b100, 0b000, 0b000, 0b100,
    0b010, 0b010, 0b100, 0b000, 0b000, 0b100, 0b010,
];
