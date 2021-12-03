use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    iter::FromIterator,
};

pub trait Neighbors {
    fn neighbours(&self) -> Vec<Self>
    where
        Self: Sized;

    fn activate(active: bool, neighbours: usize) -> bool {
        matches!((active, neighbours), (true, 2) | (.., 3))
    }
}

impl Neighbors for (i32, i32) {
    fn neighbours(&self) -> Vec<Self> {
        let &(x, y) = self;
        vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    }
}

impl Neighbors for (i32, i32, i32) {
    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = Vec::with_capacity(3 * 3 * 3 - 1);
        let &(x, y, z) = self;

        for &delta_x in &[-1, 0, 1] {
            for &delta_y in &[-1, 0, 1] {
                for &delta_z in &[-1, 0, 1] {
                    if delta_x == 0 && delta_y == 0 && delta_z == 0 {
                        continue;
                    }
                    neighbours.push((x + delta_x, y + delta_y, z + delta_z));
                }
            }
        }

        neighbours
    }
}

impl Neighbors for (i32, i32, i32, i32) {
    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = Vec::with_capacity(3 * 3 * 3 * 3 - 1);
        let &(x, y, z, w) = self;

        for &delta_x in &[-1, 0, 1] {
            for &delta_y in &[-1, 0, 1] {
                for &delta_z in &[-1, 0, 1] {
                    for &delta_w in &[-1, 0, 1] {
                        if delta_x == 0 && delta_y == 0 && delta_z == 0 && delta_w == 0 {
                            continue;
                        }
                        neighbours.push((x + delta_x, y + delta_y, z + delta_z, w + delta_w));
                    }
                }
            }
        }

        neighbours
    }
}

pub struct Board<Cell>(HashSet<Cell>);

impl<Cell: Eq + Hash> FromIterator<Cell> for Board<Cell> {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<Cell: Neighbors + Eq + Hash> Board<Cell> {
    fn neighbour_counts(&self) -> HashMap<Cell, usize> {
        let mut ncnts = HashMap::new();
        for cell in self.0.iter().flat_map(Neighbors::neighbours) {
            *ncnts.entry(cell).or_insert(0) += 1;
        }
        ncnts
    }

    pub fn next_generation(self) -> Self {
        self.neighbour_counts()
            .into_iter()
            .filter_map(|(cell, cnt)| {
                if Cell::activate(self.0.contains(&cell), cnt) {
                    Some(cell)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn alive_count(&self) -> usize {
        self.0.len()
    }
}

pub fn game_of_life<Cell: Neighbors + Eq + Hash>(
    mut board: Board<Cell>,
    iterations: usize,
) -> Board<Cell> {
    for _ in 0..iterations {
        board = board.next_generation();
    }
    board
}

impl fmt::Display for Board<(i32, i32)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scan_x = self.0.iter().map(|(x, _)| x);
        let scan_y = self.0.iter().map(|(_, y)| y);

        let min_x = *scan_x.clone().min().unwrap();
        let min_y = *scan_y.clone().min().unwrap();
        let max_x = *scan_x.max().unwrap();
        let max_y = *scan_y.max().unwrap();

        for y in min_y..max_y {
            for x in min_x..max_x {
                write!(f, "{}", if self.0.contains(&(x, y)) { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Board<(i32, i32, i32)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scan_x = self.0.iter().map(|(x, _, _)| x);
        let scan_y = self.0.iter().map(|(_, y, _)| y);
        let scan_z = self.0.iter().map(|(_, _, z)| z);

        let min_x = *scan_x.clone().min().unwrap();
        let min_y = *scan_y.clone().min().unwrap();
        let min_z = *scan_z.clone().min().unwrap();
        let max_x = *scan_x.max().unwrap();
        let max_y = *scan_y.max().unwrap();
        let max_z = *scan_z.max().unwrap();

        for z in min_z..=max_z {
            writeln!(f, "z={}", z)?;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    write!(
                        f,
                        "{}",
                        if self.0.contains(&(x, y, z)) {
                            "#"
                        } else {
                            "."
                        }
                    )?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
