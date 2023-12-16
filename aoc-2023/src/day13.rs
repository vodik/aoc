use nom::{
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    error::Error,
    multi::separated_list1,
    sequence::terminated,
    Finish, IResult,
};

#[derive(Debug)]
pub struct Grid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<u8>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn slice(&self, horizontal: &mut Vec<u64>, vertical: &mut Vec<u64>) {
        horizontal.clear();
        horizontal.resize(self.height, 0);

        vertical.clear();
        vertical.resize(self.width, 0);

        for (position, point) in self.grid.iter().enumerate() {
            if *point == b'#' {
                vertical[position % self.width] |= 1 << (position / self.width);
                horizontal[position / self.width] |= 1 << (position % self.width);
            }
        }
    }
}

pub fn grid(gridset: &str) -> impl Fn(&str) -> IResult<&str, Grid> {
    let set = gridset.chars().collect::<Vec<_>>();

    move |input: &str| {
        let mut width = None;

        let (input, lines) = separated_list1(
            tag("\n"),
            map(take_while1(|c| set.contains(&c)), |s: &str| {
                if let Some(width) = width {
                    assert_eq!(width, s.len());
                } else {
                    width = Some(s.len());
                }
                s.as_bytes()
            }),
        )(input)?;

        let grid = lines.into_iter().flatten().copied().collect::<Vec<u8>>();
        let width = width.unwrap();
        let height = grid.len() / width;

        Ok((input, Grid::new(grid, width, height)))
    }
}

fn parse_grids(input: &str) -> IResult<&str, Vec<Grid>> {
    terminated(separated_list1(tag("\n\n"), grid("#.")), tag("\n"))(input)
}

pub fn parse_input(input: &str) -> Vec<Grid> {
    match all_consuming(parse_grids)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn find_reflection(slices: &[u64]) -> Option<usize> {
    (1..slices.len()).find_map(|pivot| {
        let (left, right) = slices.split_at(pivot);
        left.iter()
            .rev()
            .zip(right)
            .all(|(&a, &b)| a == b)
            .then_some(pivot)
    })
}

pub fn part1(input: &[Grid]) -> usize {
    let mut horizontal = Vec::with_capacity(64);
    let mut vertical = Vec::with_capacity(64);

    input
        .iter()
        .flat_map(|grid| {
            grid.slice(&mut horizontal, &mut vertical);
            find_reflection(&horizontal)
                .map(|score| score * 100)
                .or_else(|| find_reflection(&vertical))
        })
        .sum()
}

fn find_smudged_reflection(slices: &[u64]) -> Option<usize> {
    'outer: for pivot in 1..slices.len() {
        let (left, right) = slices.split_at(pivot);

        let mut distortion_found = false;
        for (&a, &b) in left.iter().rev().zip(right) {
            match (a == b, distortion_found) {
                (true, _) => continue,
                (false, false) if (a ^ b).count_ones() == 1 => distortion_found = true,
                _ => continue 'outer,
            }
        }

        if distortion_found {
            return Some(pivot);
        }
    }
    None
}

pub fn part2(input: &[Grid]) -> usize {
    let mut horizontal = Vec::with_capacity(64);
    let mut vertical = Vec::with_capacity(64);

    input
        .iter()
        .flat_map(|grid| {
            grid.slice(&mut horizontal, &mut vertical);
            find_smudged_reflection(&horizontal)
                .map(|score| score * 100)
                .or_else(|| find_smudged_reflection(&vertical))
        })
        .sum()
}
