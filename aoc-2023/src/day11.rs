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

fn calculate_universe_expansion(map: &[Tile]) -> (Vec<usize>, Vec<usize>) {
    let scale_x = (0..WIDTH)
        .filter(|&x| (0..WIDTH).all(|y| map[WIDTH * y + x] == Tile::Empty))
        .collect();
    let scale_y = (0..WIDTH)
        .filter(|&y| (0..WIDTH).all(|x| map[WIDTH * y + x] == Tile::Empty))
        .collect();
    (scale_x, scale_y)
}

fn iter_galaxies_x(map: &[Tile]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..WIDTH).filter_map(|x| {
        let count = (0..WIDTH)
            .filter(|y| map[WIDTH * y + x] == Tile::Galaxy)
            .count();
        (count > 0).then_some((x, count))
    })
}

fn iter_galaxies_y(map: &[Tile]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..WIDTH).filter_map(|y| {
        let count = (0..WIDTH)
            .filter(|x| map[WIDTH * y + x] == Tile::Galaxy)
            .count();
        (count > 0).then_some((y, count))
    })
}

fn calculate_distances<const SCALE: usize>(
    (d1, count1): (usize, usize),
    (d2, count2): (usize, usize),
    scales: &[usize],
) -> usize {
    let expansion_factor = scales
        .iter()
        .filter(|&position| (d1 + 1..d2).contains(position))
        .count();
    (d2 - d1 + expansion_factor * (SCALE - 1)) * count1 * count2
}

pub fn part1(map: &[Tile]) -> usize {
    let (scale_x, scale_y) = calculate_universe_expansion(map);

    let galaxies_x = iter_galaxies_x(map).collect::<Vec<_>>();
    let dx = pairs(&galaxies_x)
        .map(|(a, b)| calculate_distances::<2>(a, b, &scale_x))
        .sum::<usize>();

    let galaxies_y = iter_galaxies_y(map).collect::<Vec<_>>();
    let dy = pairs(&galaxies_y)
        .map(|(a, b)| calculate_distances::<2>(a, b, &scale_y))
        .sum::<usize>();

    dx + dy
}

pub fn part2(map: &[Tile]) -> usize {
    let (scale_x, scale_y) = calculate_universe_expansion(map);

    let galaxies_x = iter_galaxies_x(map).collect::<Vec<_>>();
    let dx = pairs(&galaxies_x)
        .map(|(a, b)| calculate_distances::<1_000_000>(a, b, &scale_x))
        .sum::<usize>();

    let galaxies_y = iter_galaxies_y(map).collect::<Vec<_>>();
    let dy = pairs(&galaxies_y)
        .map(|(a, b)| calculate_distances::<1_000_000>(a, b, &scale_y))
        .sum::<usize>();

    dx + dy
}
