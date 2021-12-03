use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

pub type Grid = (Vec<u8>, (usize, usize));

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

        Ok((input, (grid, (width, height))))
    }
}

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

pub fn range<T: FromStr>(input: &str) -> IResult<&str, (T, T)> {
    separated_pair(number, tag("-"), number)(input)
}
