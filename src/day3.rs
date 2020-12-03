use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Tile {
    Tree,
    Empty,
}

impl Tile {
    fn is_tree(&self) -> bool {
        match self {
            Tile::Tree => true,
            Tile::Empty => false,
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn path(&self, right: usize, down: usize) -> usize {
        (0..)
            .map(|step| {
                self.tiles
                    .get(step * down * self.width + step * right % self.width)
            })
            .take_while(|tile| tile.is_some())
            .filter(|tile| tile.unwrap().is_tree())
            .count()
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
