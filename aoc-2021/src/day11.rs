use std::iter;

const WIDTH: u32 = 10;

pub fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|char| u8::try_from(char.to_digit(10).unwrap()).unwrap())
        })
        .collect()
}

struct Map {
    map: Vec<u8>,
    stack: Vec<u32>,
}

impl FromIterator<u8> for Map {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self {
            map: iter.into_iter().collect(),
            stack: Vec::with_capacity(350),
        }
    }
}

fn neighbours(point: u32) -> [Option<u32>; 8] {
    [
        point
            .checked_sub(1)
            .filter(|p| p % WIDTH != WIDTH - 1)
            .and_then(|p| p.checked_sub(WIDTH)),
        point.checked_sub(WIDTH),
        point
            .checked_add(1)
            .filter(|p| p % WIDTH != 0)
            .and_then(|p| p.checked_sub(WIDTH)),
        point.checked_sub(1).filter(|p| p % WIDTH != WIDTH - 1),
        point.checked_add(1).filter(|p| p % WIDTH != 0),
        point
            .checked_sub(1)
            .filter(|p| p % WIDTH != WIDTH - 1)
            .and_then(|p| p.checked_add(WIDTH))
            .filter(|&p| p < (WIDTH * WIDTH) as u32),
        point
            .checked_add(WIDTH)
            .filter(|&p| p < (WIDTH * WIDTH) as u32),
        point
            .checked_add(1)
            .filter(|p| p % WIDTH != 0)
            .and_then(|p| p.checked_add(WIDTH))
            .filter(|&p| p < (WIDTH * WIDTH) as u32),
    ]
}

impl Map {
    fn step(&mut self) -> usize {
        self.stack.extend((0..(WIDTH * WIDTH)).filter(|&point| {
            let energy = &mut self.map[point as usize];
            *energy += 1;
            *energy > 9
        }));

        let mut events = 0;
        let mut flashed = vec![false; (WIDTH * WIDTH) as usize];
        while let Some(point) = self.stack.pop() {
            if flashed[point as usize] {
                continue;
            }

            let energy = &mut self.map[point as usize];
            *energy += 1;

            if *energy > 9 {
                *energy = 0;
                events += 1;
                flashed[point as usize] = true;
                self.stack
                    .extend(neighbours(point).iter().flatten().copied());
            }
        }

        events
    }
}

pub fn part1(input: &[u8]) -> usize {
    let mut map: Map = input.iter().copied().collect();
    iter::from_fn(move || Some(map.step())).take(100).sum()
}

pub fn part2(input: &[u8]) -> usize {
    let mut map: Map = input.iter().copied().collect();
    iter::from_fn(move || Some(map.step()))
        .take_while(|&flashes| flashes != (WIDTH * WIDTH) as usize)
        .count()
        + 1
}
