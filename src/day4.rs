use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{alphanumeric1, digit1, one_of},
    combinator::{map, map_res, recognize},
    error::Error,
    multi::{many1, separated_list1},
    Finish, IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

enum Measurement {
    Metric(u32),
    Imperial(u32),
}

fn parse_measurement(input: &str) -> IResult<&str, Measurement> {
    let (input, value) = map_res(digit1, str::parse)(input)?;
    alt((
        map(tag("cm"), move |_| Measurement::Metric(value)),
        map(tag("in"), move |_| Measurement::Imperial(value)),
    ))(input)
}

impl FromStr for Measurement {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_measurement(s).finish() {
            Ok((_, measurement)) => Ok(measurement),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
struct HexColor(String);

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn parse_hex(input: &str) -> IResult<&str, HexColor> {
    let (input, _) = tag("#")(input)?;
    map(take_while_m_n(6, 6, is_hex_digit), |s: &str| {
        HexColor(s.to_string())
    })(input)
}

impl FromStr for HexColor {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_hex(s).finish() {
            Ok((_, color)) => Ok(color),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
enum Color {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        map(tag("amb"), |_| Color::Amber),
        map(tag("blu"), |_| Color::Blue),
        map(tag("brn"), |_| Color::Brown),
        map(tag("gry"), |_| Color::Grey),
        map(tag("grn"), |_| Color::Green),
        map(tag("hzl"), |_| Color::Hazel),
        map(tag("oth"), |_| Color::Other),
    ))(input)
}

impl FromStr for Color {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_color(s).finish() {
            Ok((_, color)) => Ok(color),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
struct Pid(u32);

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_pid(input: &str) -> IResult<&str, Pid> {
    map_res(take_while_m_n(9, 9, is_digit), |s: &str| s.parse().map(Pid))(input)
}

impl FromStr for Pid {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_pid(s).finish() {
            Ok((_, pid)) => Ok(pid),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Passport(HashMap<String, String>);

impl Passport {
    fn birth_year(&self) -> Option<u32> {
        self.0.get("byr").and_then(|field| field.parse().ok())
    }

    fn issue_year(&self) -> Option<u32> {
        self.0.get("iyr").and_then(|field| field.parse().ok())
    }

    fn expiration_year(&self) -> Option<u32> {
        self.0.get("eyr").and_then(|field| field.parse().ok())
    }

    fn height(&self) -> Option<Measurement> {
        self.0.get("hgt").and_then(|field| field.parse().ok())
    }

    fn hair_color(&self) -> Option<HexColor> {
        self.0.get("hcl").and_then(|field| field.parse().ok())
    }

    fn eye_color(&self) -> Option<Color> {
        self.0.get("ecl").and_then(|field| field.parse().ok())
    }

    fn passport_id(&self) -> Option<Pid> {
        self.0.get("pid").and_then(|field| field.parse().ok())
    }

    fn is_valid(&self) -> bool {
        self.0.contains_key("byr")
            && self.0.contains_key("iyr")
            && self.0.contains_key("eyr")
            && self.0.contains_key("hgt")
            && self.0.contains_key("hcl")
            && self.0.contains_key("ecl")
            && self.0.contains_key("pid")
    }

    fn is_valid_strict(&self) -> bool {
        self.birth_year()
            .map_or(false, |year| (1920..=2002).contains(&year))
            && self
                .issue_year()
                .map_or(false, |year| (2010..=2020).contains(&year))
            && self
                .expiration_year()
                .map_or(false, |year| (2020..=2030).contains(&year))
            && self.height().map_or(false, |value| match value {
                Measurement::Metric(height) => (150..=193).contains(&height),
                Measurement::Imperial(height) => (59..=76).contains(&height),
            })
            && self.hair_color().is_some()
            && self.eye_color().is_some()
            && self.passport_id().is_some()
    }
}

fn parse_field(input: &str) -> IResult<&str, (String, String)> {
    let (input, field) = alphanumeric1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = recognize(many1(alt((alphanumeric1, tag("#")))))(input)?;

    Ok((input, (field.to_string(), value.to_string())))
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let (input, pairs) = separated_list1(one_of(" \n"), parse_field)(input)?;

    Ok((input, Passport(pairs.into_iter().collect())))
}

fn parse_passports(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(tag("\n\n"), parse_passport)(input)
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Result<Vec<Passport>, Error<String>> {
    match parse_passports(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
}

#[aoc(day4, part1)]
fn part1(data: &[Passport]) -> usize {
    data.iter().filter(|passport| passport.is_valid()).count()
}

#[aoc(day4, part2)]
fn part2(data: &[Passport]) -> usize {
    data.iter()
        .filter(|passport| passport.is_valid_strict())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_iyr() {
        assert_eq!(
            parse_field("iyr:2010"),
            Ok(("", ("iyr".to_string(), "2010".to_string())))
        );
    }

    #[test]
    fn can_parse_hcl() {
        assert_eq!(
            parse_field("hcl:#6b5442"),
            Ok(("", ("hcl".to_string(), "#6b5442".to_string())))
        );
    }

    #[test]
    fn can_parse_passport() {
        assert_eq!(
            parse_passport("hgt:168cm\niyr:2018 hcl:#ceb3a1\npid:116783406"),
            Ok((
                "",
                Passport(hashmap! {
                    "hgt".to_string() => "168cm".to_string(),
                    "iyr".to_string() => "2018".to_string(),
                    "hcl".to_string() => "#ceb3a1".to_string(),
                    "pid".to_string() => "116783406".to_string(),
                })
            ))
        )
    }

    #[test]
    fn can_parse_many_passports() {
        assert_eq!(
            parse_passports("hgt:168cm\niyr:2018\n\nhcl:#ceb3a1\npid:116783406"),
            Ok((
                "",
                vec![
                    Passport(hashmap! {
                        "hgt".to_string() => "168cm".to_string(),
                        "iyr".to_string() => "2018".to_string(),
                    }),
                    Passport(hashmap! {
                        "hcl".to_string() => "#ceb3a1".to_string(),
                        "pid".to_string() => "116783406".to_string(),
                    })
                ]
            ))
        )
    }
}
