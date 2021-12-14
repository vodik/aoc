use nom::{
    bytes::complete::{tag, take_while},
    combinator::{all_consuming, map, opt},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

fn ord(c: u8) -> usize {
    (c - b'A') as usize
}

fn encode(a: u8, b: u8) -> usize {
    ord(a) * 26 + ord(b)
}

fn word(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        take_while(move |c: char| c.is_ascii_uppercase()),
        |word: &str| word.bytes().collect(),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (usize, usize, usize)> {
    map(separated_pair(word, tag(" -> "), word), |(pair, result)| {
        (ord(pair[0]), ord(pair[1]), ord(result[0]))
    })(input)
}

fn parse_initial(input: &str) -> IResult<&str, Vec<u8>> {
    word(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    separated_list1(tag("\n"), parse_rule)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<u8>, Vec<(usize, usize, usize)>)> {
    terminated(
        separated_pair(parse_initial, tag("\n\n"), parse_rules),
        opt(tag("\n")),
    )(input)
}

pub fn parse_input(input: &str) -> (Vec<u8>, Vec<(usize, usize, usize)>) {
    match all_consuming(parse_file)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn init_pairs(input: &[u8]) -> [usize; 26 * 26] {
    let mut pairs = [0; 26 * 26];

    for window in input.windows(2) {
        let key = encode(window[0], window[1]);
        pairs[key] += 1
    }

    pairs
}

fn step(pairs: &[usize], rules: &[(usize, usize, usize)]) -> [usize; 26 * 26] {
    let mut new_pairs = [0; 26 * 26];

    for (a, b, c) in rules {
        let count = pairs[a * 26 + b];
        new_pairs[a * 26 + c] += count;
        new_pairs[c * 26 + b] += count;
    }

    new_pairs
}

fn solve<const N: usize>(input: &[u8], rules: &[(usize, usize, usize)]) -> usize {
    let mut counts = [0usize; 26];
    counts[ord(input[0])] = 1;

    (0..N)
        .fold(init_pairs(input), |pairs, _| step(&pairs, rules))
        .iter()
        .enumerate()
        .for_each(|(pair, count)| {
            counts[pair % 26] += count;
        });

    counts.iter().max().unwrap() - counts.iter().filter(|&&count| count > 0).min().unwrap()
}

pub fn part1(input: &(Vec<u8>, Vec<(usize, usize, usize)>)) -> usize {
    solve::<10>(&input.0, &input.1)
}

pub fn part2(input: &(Vec<u8>, Vec<(usize, usize, usize)>)) -> usize {
    solve::<40>(&input.0, &input.1)
}
