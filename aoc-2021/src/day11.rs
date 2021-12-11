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

struct Map(Vec<u8>);

impl FromIterator<u8> for Map {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
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
        for cell in self.0.iter_mut() {
            *cell += 1;
        }

        let mut flashes = 0;
        let mut has_flashed = vec![false; (WIDTH * WIDTH) as usize];
        for point in 0..(WIDTH * WIDTH) {
            let energy = self.0[point as usize];
            if has_flashed[point as usize] || energy != 10 {
                continue;
            }

            let mut stack = Vec::with_capacity(100);
            stack.push(point);

            while let Some(point) = stack.pop() {
                if has_flashed[point as usize] {
                    continue;
                }

                let energy = &mut self.0[point as usize];
                *energy += 1;

                if *energy >= 10 {
                    has_flashed[point as usize] = true;
                    flashes += 1;
                    *energy = 0;

                    stack.extend(neighbours(point).iter().copied().flatten())
                }
            }
        }

        flashes
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
