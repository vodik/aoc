use std::{ops::Range, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};

#[derive(Debug)]
struct Transform {
    range: Range<u64>,
    transform: i64,
}

impl Transform {
    fn new(dest: u64, source: u64, length: u64) -> Self {
        Self {
            range: (source..source + length),
            transform: dest as i64 - source as i64,
        }
    }

    fn apply(&self, input: u64) -> Option<u64> {
        self.range
            .contains(&input)
            .then(|| (input as i64 + self.transform).try_into().unwrap())
    }
}

#[derive(Debug)]
struct Map(Vec<Transform>);

impl Map {
    fn get(&self, input: u64) -> u64 {
        self.0
            .iter()
            .find_map(|transform| transform.apply(input))
            .unwrap_or(input)
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, FromStr::from_str)(input)
}

fn parse_transform(input: &str) -> IResult<&str, Transform> {
    map(
        tuple((
            terminated(number, tag(" ")),
            terminated(number, tag(" ")),
            number,
        )),
        |(base, output, length)| Transform::new(base, output, length),
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    map(separated_list1(tag("\n"), parse_transform), Map)(input)
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    map(
        terminated(
            tuple((
                preceded(tag("seeds: "), separated_list1(tag(" "), number)),
                preceded(tag("\n\nseed-to-soil map:\n"), parse_map),
                preceded(tag("\n\nsoil-to-fertilizer map:\n"), parse_map),
                preceded(tag("\n\nfertilizer-to-water map:\n"), parse_map),
                preceded(tag("\n\nwater-to-light map:\n"), parse_map),
                preceded(tag("\n\nlight-to-temperature map:\n"), parse_map),
                preceded(tag("\n\ntemperature-to-humidity map:\n"), parse_map),
                preceded(tag("\n\nhumidity-to-location map:\n"), parse_map),
            )),
            tag("\n"),
        ),
        |(
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        )| Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )(input)
}

pub fn parse_input(input: &str) -> Almanac {
    match all_consuming(parse_almanac)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

pub fn part1(input: &Almanac) -> u64 {
    input
        .seeds
        .iter()
        .map(|&seed| {
            let soil = input.seed_to_soil_map.get(seed);
            let fertilizer = input.soil_to_fertilizer_map.get(soil);
            let water = input.fertilizer_to_water.get(fertilizer);
            let light = input.water_to_light.get(water);
            let temperature = input.light_to_temperature.get(light);
            let humidity = input.temperature_to_humidity.get(temperature);
            input.humidity_to_location.get(humidity)
        })
        .min()
        .unwrap()
}

pub fn part2(input: &Almanac) -> u64 {
    input
        .seeds
        .chunks(2)
        .flat_map(|range| (range[0]..range[0] + range[1]))
        .map(|seed| {
            let soil = input.seed_to_soil_map.get(seed);
            let fertilizer = input.soil_to_fertilizer_map.get(soil);
            let water = input.fertilizer_to_water.get(fertilizer);
            let light = input.water_to_light.get(water);
            let temperature = input.light_to_temperature.get(light);
            let humidity = input.temperature_to_humidity.get(temperature);
            input.humidity_to_location.get(humidity)
        })
        .min()
        .unwrap()
}
