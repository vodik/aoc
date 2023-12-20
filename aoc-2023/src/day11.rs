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

fn galaxies_x<'a>(
    map: &'a [Tile],
    prefix_sums: &'a [usize],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..WIDTH).filter_map(move |x| {
        let count = (0..WIDTH)
            .filter(|y| map[WIDTH * y + x] == Tile::Galaxy)
            .count();
        (count > 0).then_some((prefix_sums[x], count))
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

fn galaxies_y<'a>(
    map: &'a [Tile],
    prefix_sums: &'a [usize],
) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..WIDTH).filter_map(move |y| {
        let count = (0..WIDTH)
            .filter(|x| map[WIDTH * y + x] == Tile::Galaxy)
            .count();
        (count > 0).then_some((prefix_sums[y], count))
    })
}

fn pairs<T: Copy>(slice: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    (0..slice.len()).flat_map(move |i| (i + 1..slice.len()).map(move |j| (slice[i], slice[j])))
}

pub fn solution(map: &[Tile], scale: usize) -> usize {
    let mut prefix_sums = Vec::with_capacity(140);
    let mut galaxies = Vec::with_capacity(140);

    prefix_sums.extend(expansion_prefix_sums_x(map, scale));
    galaxies.extend(galaxies_x(map, &prefix_sums));
    let dx = pairs(&galaxies)
        .map(|((d1, count1), (d2, count2))| (d2 - d1) * count1 * count2)
        .sum::<usize>();

    prefix_sums.clear();
    galaxies.clear();

    prefix_sums.extend(expansion_prefix_sums_y(map, scale));
    galaxies.extend(galaxies_y(map, &prefix_sums));
    let dy = pairs(&galaxies)
        .map(|((d1, count1), (d2, count2))| (d2 - d1) * count1 * count2)
        .sum::<usize>();

    dx + dy
}

pub fn part1(map: &[Tile]) -> usize {
    solution(map, 2)
}

pub fn part2(map: &[Tile]) -> usize {
    solution(map, 1_000_000)
}
