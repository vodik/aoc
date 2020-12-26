use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::separated_pair, IResult,
};
use std::str::FromStr;

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

pub fn range<T: FromStr>(input: &str) -> IResult<&str, (T, T)> {
    separated_pair(number, tag("-"), number)(input)
}
