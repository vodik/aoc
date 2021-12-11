use std::collections::{HashMap, HashSet};

const WIDTH: i32 = 10;
type Point = (i32, i32);

pub fn parse_input(input: &str) -> Vec<(Point, u8)> {
    let mut map = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            map.push(((x as i32, y as i32), char.to_digit(10).unwrap() as u8));
        }
    }
    map
}

struct Map(HashMap<Point, u8>);

impl FromIterator<(Point, u8)> for Map {
    fn from_iter<T: IntoIterator<Item = (Point, u8)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Map {
    fn step(&mut self) -> usize {
        for cell in self.0.values_mut() {
            *cell += 1;
        }

        let mut flashes = 0;
        let mut has_flashed = HashSet::new();
        for x in 0..WIDTH {
            for y in 0..WIDTH {
                let point = (x, y);

                let energy = self.0.get(&point).unwrap();
                if has_flashed.contains(&point) || *energy != 10 {
                    continue;
                }

                let mut stack = Vec::with_capacity(100);
                stack.push(point);

                while let Some(point) = stack.pop() {
                    if has_flashed.contains(&point) {
                        continue;
                    }

                    if let Some(energy) = self.0.get_mut(&point) {
                        *energy = u8::min(*energy + 1, 10);
                        if *energy == 10 {

                            *energy = 0;
                            flashes += 1;
                            has_flashed.insert(point);

                            for i in -1..=1 {
                                for j in -1..=1 {
                                    let new_point = (point.0 + i, point.1 + j);
                                    if new_point != point && self.0.get(&new_point).is_some() {
                                        stack.push(new_point);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        flashes
    }
}

pub fn part1(input: &[(Point, u8)]) -> usize {
    let map: Map = input.iter().copied().collect();
    (0..100).scan(map, |map, _| Some(map.step())).sum()
}

pub fn part2(input: &[(Point, u8)]) -> usize {
    let map: Map = input.iter().copied().collect();
    (0..)
        .scan(map, |map, _| Some(map.step()))
        .take_while(|&flashes| flashes != (WIDTH * WIDTH) as usize)
        .count()
        + 1
}
