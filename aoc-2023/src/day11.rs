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

fn pairs<T: Copy>(slice: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    (0..slice.len()).flat_map(move |i| (i + 1..slice.len()).map(move |j| (slice[i], slice[j])))
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
    let scale_x = (0..WIDTH)
        .filter(|&x| (0..WIDTH).all(|y| map[WIDTH * y + x] == Tile::Empty))
        .collect();
    let scale_y = (0..WIDTH)
        .filter(|&y| (0..WIDTH).all(|x| map[WIDTH * y + x] == Tile::Empty))
        .collect();
    (scale_x, scale_y)
}

fn calculate_distance<const SCALE: usize>(
    (a, b): (usize, usize),
    scale_x: &[usize],
    scale_y: &[usize],
) -> usize {
    let (ax, ay) = decompose(a);
    let (bx, by) = decompose(b);

    let (min_x, max_x) = (ax.min(bx), ax.max(bx));
    let (min_y, max_y) = (ay.min(by), ay.max(by));

    let dx = max_x - min_x;
    let x_expansion_factor = scale_x
        .iter()
        .filter(|&position| (min_x + 1..max_x).contains(position))
        .count();

    let dy = max_y - min_y;
    let y_expansion_factor = scale_y
        .iter()
        .filter(|&position| (min_y + 1..max_y).contains(position))
        .count();

    dx + dy + (x_expansion_factor + y_expansion_factor) * (SCALE - 1)
}

pub fn part1(map: &[Tile]) -> usize {
    let (scale_x, scale_y) = calculate_universe_expansion(map);
    let galaxies = iter_galaxies(map).collect::<Vec<_>>();
    pairs(&galaxies)
        .map(|pair| calculate_distance::<2>(pair, &scale_x, &scale_y))
        .sum()
}

pub fn part2(map: &[Tile]) -> usize {
    let (scale_x, scale_y) = calculate_universe_expansion(map);
    let galaxies = iter_galaxies(map).collect::<Vec<_>>();
    pairs(&galaxies)
        .map(|pair| calculate_distance::<1_000_000>(pair, &scale_x, &scale_y))
        .sum()
}
