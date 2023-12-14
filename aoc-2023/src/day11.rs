const WIDTH: usize = 140;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Galaxy,
    Empty,
}

pub fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .flat_map(|line| {
            line.bytes().map(|c| match c {
                b'#' => Tile::Galaxy,
                b'.' => Tile::Empty,
                _ => unreachable!(),
            })
        })
        .collect()
}

fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> + '_ {
    (0..slice.len()).flat_map(move |i| (i + 1..slice.len()).map(move |j| (&slice[i], &slice[j])))
}

fn decompose(point: usize) -> (usize, usize) {
    (point % WIDTH, point / WIDTH)
}

fn iter_galaxies(map: &[Tile]) -> impl Iterator<Item = usize> + '_ {
    map.iter()
        .enumerate()
        .filter_map(|(position, &tile)| (tile == Tile::Galaxy).then_some(position))
}

fn calculate_universe_expansion(map: &[Tile]) -> (Vec<usize>, Vec<usize>) {
    let mut scale_x = Vec::with_capacity(10);
    let mut scale_y = Vec::with_capacity(10);
    for rank in 0..WIDTH {
        if (0..WIDTH).all(|y| map[WIDTH * y + rank] == Tile::Empty) {
            scale_x.push(rank);
        }
        if (0..WIDTH).all(|x| map[WIDTH * rank + x] == Tile::Empty) {
            scale_y.push(rank);
        }
    }
    (scale_x, scale_y)
}

fn calculate_distance(
    (&a, &b): (&usize, &usize),
    scale_x: &[usize],
    scale_y: &[usize],
    scale: usize,
) -> usize {
    let a = decompose(a);
    let b = decompose(b);

    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    let dx = max_x - min_x;
    let dy = max_y - min_y;

    let x_expansion_factor = scale_x
        .iter()
        .filter(|position| (min_x..max_x).contains(*position))
        .count();

    let y_expansion_factor = scale_y
        .iter()
        .filter(|position| (min_y..max_y).contains(*position))
        .count();

    dx + dy + (x_expansion_factor + y_expansion_factor) * (scale - 1)
}

pub fn part1(map: &[Tile]) -> usize {
    let (scale_x, scale_y) = calculate_universe_expansion(map);
    let galaxies = iter_galaxies(map).collect::<Vec<_>>();
    pairs(&galaxies)
        .map(|pair| calculate_distance(pair, &scale_x, &scale_y, 2))
        .sum()
}

pub fn part2(map: &[Tile]) -> usize {
    let (scale_x, scale_y) = calculate_universe_expansion(map);
    let galaxies = iter_galaxies(map).collect::<Vec<_>>();
    pairs(&galaxies)
        .map(|pair| calculate_distance(pair, &scale_x, &scale_y, 1_000_000))
        .sum()
}
