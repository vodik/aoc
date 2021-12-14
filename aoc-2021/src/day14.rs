use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while},
    combinator::{all_consuming, map, opt},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

pub struct Rule {
    pair: [u8; 2],
    result: u8,
}

fn word(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        take_while(move |c: char| c.is_ascii_uppercase()),
        |word: &str| word.bytes().collect(),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(separated_pair(word, tag(" -> "), word), |(x, y)| Rule {
        pair: x.try_into().unwrap(),
        result: y[0],
    })(input)
}

fn parse_initial(input: &str) -> IResult<&str, Vec<u8>> {
    word(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag("\n"), parse_rule)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<u8>, Vec<Rule>)> {
    terminated(
        separated_pair(parse_initial, tag("\n\n"), parse_rules),
        opt(tag("\n")),
    )(input)
}

pub fn parse_input(input: &str) -> (Vec<u8>, Vec<Rule>) {
    match all_consuming(parse_file)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn pairs(input: &[u8]) -> HashMap<u16, usize> {
    let mut pairs = HashMap::new();

    for window in input.windows(2) {
        let key = (window[0] as u16) << 8 | window[1] as u16;
        pairs
            .entry(key)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pairs
}

fn step(pairs: &HashMap<u16, usize>, rules: &HashMap<u16, u8>) -> HashMap<u16, usize> {
    let mut new_pairs: HashMap<u16, usize> = HashMap::new();

    for (pair, &count) in pairs {
        let mid = rules[pair];
        let first = pair & 0xff00 | mid as u16;
        let second = pair & 0xff | (mid as u16) << 8;
        *new_pairs.entry(first).or_default() += count;
        *new_pairs.entry(second).or_default() += count;
    }

    new_pairs
}

fn freq(pairs: &HashMap<u16, usize>, first: u8) -> usize {
    let mut counts = [0usize; 26];
    counts[(first - b'A') as usize] = 1;

    for (pair, &count) in pairs {
        let b = u8::try_from(pair & 0xff).unwrap();
        counts[(b - b'A') as usize] += count;
    }
    counts.iter().max().unwrap() - counts.iter().filter(|&&x| x > 0).min().unwrap()
}

fn solve<const N: usize>((input, rules): &(Vec<u8>, Vec<Rule>)) -> usize {
    let rules: HashMap<u16, u8> = rules
        .iter()
        .map(|&Rule { pair, result }| {
            let encoded = (pair[0] as u16) << 8 | pair[1] as u16;
            (encoded, result)
        })
        .collect();

    let mut pairs = pairs(input);
    for _ in 0..N {
        pairs = step(&pairs, &rules);
    }

    freq(&pairs, input[0])
}

pub fn part1(input: &(Vec<u8>, Vec<Rule>)) -> usize {
    solve::<10>(input)
}

pub fn part2(input: &(Vec<u8>, Vec<Rule>)) -> usize {
    solve::<40>(input)
}
