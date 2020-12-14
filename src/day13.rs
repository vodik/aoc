use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Bus {
    Bus(u64),
    OutOfService,
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Result<(u64, Vec<Bus>), ParseIntError> {
    let mut lines = input.lines();

    let timestamp = lines.next().unwrap().parse()?;
    let schedule = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            if s == "x" {
                Ok(Bus::OutOfService)
            } else {
                Ok(Bus::Bus(s.parse()?))
            }
        })
        .collect::<Result<_, _>>()?;

    Ok((timestamp, schedule))
}

#[aoc(day13, part1)]
fn part1(data: &(u64, Vec<Bus>)) -> Option<u64> {
    let (start, schedule) = data;
    let schedule = schedule
        .iter()
        .filter_map(|entry| match entry {
            Bus::OutOfService => None,
            Bus::Bus(id) => Some(*id),
        })
        .collect::<Vec<_>>();

    (*start..).find_map(|timestamp| {
        schedule
            .iter()
            .find(|id| timestamp % *id == 0)
            .map(|id| (timestamp - start) * id)
    })
}

#[aoc(day13, part2)]
fn part2(data: &(u64, Vec<Bus>)) -> u64 {
    let (_, schedule) = data;
    let offsets = schedule
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| match entry {
            Bus::OutOfService => None,
            Bus::Bus(id) => Some((idx as u64 % *id, *id)),
        })
        .collect::<Vec<_>>();

    let lcd = offsets.iter().map(|(_, id)| *id).product::<u64>();
    let sum = offsets
        .iter()
        .map(|(offset, id)| {
            let step = lcd / id;
            let mut timestamp = step;

            while timestamp % id != 1 {
                timestamp += step;
            }

            timestamp * (id - offset)
        })
        .sum::<u64>();

    sum % lcd
}

#[aoc(day13, part2, Iterative)]
fn part2_alt(data: &(u64, Vec<Bus>)) -> u64 {
    let (_, schedule) = data;
    let offsets = schedule
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| match entry {
            Bus::OutOfService => None,
            Bus::Bus(id) => Some((idx as u64 % *id, *id)),
        })
        .collect::<Vec<_>>();

    let mut timestamp = 0;
    let (_, mut step) = offsets[0];

    for (offset, id) in &offsets[1..] {
        while timestamp % id != id - offset {
            timestamp += step;
        }
        step *= id;
    }

    timestamp
}
