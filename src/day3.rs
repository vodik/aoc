use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Tile {
    Tree,
    Empty,
}

#[derive(Debug)]
struct Map {
    width: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn path(&self, right: usize, down: usize) -> usize {
        let mut acc = 0;
        let mut row = 0;
        let mut offset = 0;

        while let Some(tile) = self.tiles.get(row * self.width + offset) {
            acc += match tile {
                Tile::Tree => 1,
                Tile::Empty => 0,
            };

            row += down;
            offset += right;
            offset %= self.width;
        }

        acc
    }
}

#[aoc_generator(day3)]
fn parse_map(input: &str) -> Map {
    let mut width = None;

    let tiles = input
        .lines()
        .flat_map(|line| {
            if let Some(width) = width {
                assert_eq!(width, line.len())
            } else {
                width = Some(line.len());
            }

            line.chars().map(|c| match c {
                '#' => Tile::Tree,
                '.' => Tile::Empty,
                _ => unreachable!(),
            })
        })
        .collect();

    Map {
        width: width.unwrap(),
        tiles: tiles,
    }
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    map.path(3, 1)
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    map.path(1, 1) * map.path(3, 1) * map.path(5, 1) * map.path(7, 1) * map.path(1, 2)
}
