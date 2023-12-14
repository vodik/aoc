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

fn expansion_prefix_sums_x(map: &[Tile], scale: usize) -> impl Iterator<Item = usize> + '_ {
    (0..WIDTH)
        .map(|x| (0..WIDTH).all(|y| map[WIDTH * y + x] == Tile::Empty))
        .scan(0, move |dist, empty| {
            *dist += if empty { scale } else { 1 };
            Some(*dist)
        })
}

fn galaxies_x(map: &[Tile]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..WIDTH).filter_map(|x| {
        let count = (0..WIDTH)
            .filter(|y| map[WIDTH * y + x] == Tile::Galaxy)
            .count();
        (count > 0).then_some((x, count))
    })
}

fn expansion_prefix_sums_y(map: &[Tile], scale: usize) -> impl Iterator<Item = usize> + '_ {
    (0..WIDTH)
        .map(|y| (0..WIDTH).all(|x| map[WIDTH * y + x] == Tile::Empty))
        .scan(0, move |dist, empty| {
            *dist += if empty { scale } else { 1 };
            Some(*dist)
        })
}

fn galaxies_y(map: &[Tile]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..WIDTH).filter_map(|y| {
        let count = (0..WIDTH)
            .filter(|x| map[WIDTH * y + x] == Tile::Galaxy)
            .count();
        (count > 0).then_some((y, count))
    })
}

fn pairs<T: Copy>(slice: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    (0..slice.len()).flat_map(move |i| (i + 1..slice.len()).map(move |j| (slice[i], slice[j])))
}

fn calculate_distances(
    (d1, count1): (usize, usize),
    (d2, count2): (usize, usize),
    prefix_sums: &[usize],
) -> usize {
    (prefix_sums[d2] - prefix_sums[d1]) * count1 * count2
}

pub fn solution(map: &[Tile], scale: usize) -> usize {
    let prefix_sums_x = expansion_prefix_sums_x(map, scale).collect::<Vec<_>>();
    let galaxies_x = galaxies_x(map).collect::<Vec<_>>();
    let dx = pairs(&galaxies_x)
        .map(|(a, b)| calculate_distances(a, b, &prefix_sums_x))
        .sum::<usize>();

    let prefix_sums_y = expansion_prefix_sums_y(map, scale).collect::<Vec<_>>();
    let galaxies_y = galaxies_y(map).collect::<Vec<_>>();
    let dy = pairs(&galaxies_y)
        .map(|(a, b)| calculate_distances(a, b, &prefix_sums_y))
        .sum::<usize>();

    dx + dy
}

pub fn part1(map: &[Tile]) -> usize {
    solution(map, 2)
}

pub fn part2(map: &[Tile]) -> usize {
    solution(map, 1_000_000)
}
