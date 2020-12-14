use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Hash)]
struct Grid {
    width: i64,
    height: i64,
    cells: Vec<Cell>,
}

impl Grid {
    fn get_index(&self, row: i64, column: i64) -> usize {
        (row * self.width + column).try_into().unwrap()
    }

    fn neighbour_count(&self, row: i64, column: i64) -> u8 {
        let mut count = 0;
        for row_delta in [-1, 0, 1].iter().cloned() {
            for column_delta in [-1, 0, 1].iter().cloned() {
                if row_delta == 0 && column_delta == 0 {
                    continue;
                }

                let neighbour_row = row + row_delta;
                if !(0..self.height).contains(&neighbour_row) {
                    continue;
                }

                let neighbour_column = column + column_delta;
                if !(0..self.width).contains(&neighbour_column) {
                    continue;
                }

                let idx = self.get_index(neighbour_row, neighbour_column);
                if self.cells[idx] == Cell::Occupied {
                    count += 1;
                }
            }
        }
        count
    }

    fn neighbour_seen(&self, row: i64, column: i64) -> u8 {
        let mut count = 0;

        if (0..row)
            .rev()
            .map(|row| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (0..row)
            .rev()
            .zip(column + 1..self.width)
            .map(|(row, column)| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (column + 1..self.width)
            .map(|column| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (column + 1..self.width)
            .zip(row + 1..self.height)
            .map(|(column, row)| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (row + 1..self.height)
            .map(|row| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (row + 1..self.height)
            .zip((0..column).rev())
            .map(|(row, column)| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (0..column)
            .rev()
            .map(|column| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        if (0..column)
            .rev()
            .zip((0..row).rev())
            .map(|(column, row)| self.get_index(row, column))
            .find_map(|idx| match self.cells[idx] {
                Cell::Occupied => Some(true),
                Cell::Empty => Some(false),
                _ => None,
            })
            .unwrap_or(false)
        {
            count += 1;
        }

        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let neighbors = self.neighbour_count(row, col);

                let next_cell = match (cell, neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, x) if x >= 4 => Cell::Empty,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    fn tick2(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let neighbors = self.neighbour_seen(row, col);

                let next_cell = match (cell, neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, x) if x >= 5 => Cell::Empty,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

#[aoc_generator(day11)]
fn parse_grid(input: &str) -> Grid {
    let mut width = None;

    let cells = input
        .lines()
        .flat_map(|line| {
            if let Some(width) = width {
                assert_eq!(width, line.len())
            } else {
                width = Some(line.len());
            }

            line.chars().map(|c| match c {
                '.' => Cell::Floor,
                'L' => Cell::Empty,
                '#' => Cell::Occupied,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();

    let width = width.unwrap();
    let height = cells.len() / width;
    Grid {
        cells,
        width: width as i64,
        height: height as i64,
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[aoc(day11, part1)]
fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut hash = calculate_hash(&grid);

    loop {
        grid.tick();
        let new_hash = calculate_hash(&grid);
        if hash == new_hash {
            break;
        }
        hash = new_hash;
    }

    grid.cells
        .iter()
        .filter(|&cell| cell == &Cell::Occupied)
        .count()
}

#[aoc(day11, part2)]
fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut hash = calculate_hash(&grid);

    loop {
        grid.tick2();
        let new_hash = calculate_hash(&grid);
        if hash == new_hash {
            break;
        }
        hash = new_hash;
    }

    grid.cells
        .iter()
        .filter(|&cell| cell == &Cell::Occupied)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_see_8() {
        let grid = parse_grid(
            ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
        );
        assert_eq!(grid.cells[grid.get_index(4, 3)], Cell::Empty);
        assert_eq!(grid.neighbour_seen(4, 3), 8);
    }

    #[test]
    fn can_see_1() {
        let grid = parse_grid(
            ".............
.L.L.#.#.#.#.
.............",
        );
        assert_eq!(grid.cells[grid.get_index(1, 1)], Cell::Empty);
        assert_eq!(grid.neighbour_seen(1, 1), 1);
    }

    #[test]
    fn can_see_0() {
        let grid = parse_grid(
            ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
        );
        assert_eq!(grid.cells[grid.get_index(3, 3)], Cell::Empty);
        assert_eq!(grid.neighbour_seen(3, 3), 0);
    }

    #[test]
    fn can_see_0_2() {
        let grid = parse_grid(
            ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
        );
        assert_eq!(grid.cells[grid.get_index(3, 3)], Cell::Empty);
        assert_eq!(grid.neighbour_seen(3, 3), 0);
    }
}
