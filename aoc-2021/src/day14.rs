use nom::{
    bytes::complete::{tag, take_while},
    combinator::{all_consuming, map, opt},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

fn word(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        take_while(move |c: char| c.is_ascii_uppercase()),
        |word: &str| word.bytes().collect(),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (usize, usize)> {
    map(separated_pair(word, tag(" -> "), word), |(pair, result)| {
        (encode(pair[0], pair[1]), ord(result[0]))
    })(input)
}

fn parse_initial(input: &str) -> IResult<&str, Vec<u8>> {
    word(input)
}

fn parse_rules(input: &str) -> IResult<&str, [usize; 26 * 26]> {
    map(separated_list1(tag("\n"), parse_rule), |rules| {
        let mut new_rules = [0usize; 26 * 26];
        for (pair, result) in rules {
            new_rules[pair] = result;
        }
        new_rules
    })(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<u8>, [usize; 26 * 26])> {
    terminated(
        separated_pair(parse_initial, tag("\n\n"), parse_rules),
        opt(tag("\n")),
    )(input)
}

pub fn parse_input(input: &str) -> (Vec<u8>, [usize; 26 * 26]) {
    match all_consuming(parse_file)(input).finish() {
        Ok((_, output)) => Ok(output),
        Err(Error { input, code }) => Err(Error {
            input: input.to_string(),
            code,
        }),
    }
    .unwrap()
}

fn ord(c: u8) -> usize {
    (c - b'A') as usize
}

fn encode(a: u8, b: u8) -> usize {
    ord(a) * 26 + ord(b)
}

fn pairs(input: &[u8]) -> [usize; 26 * 26] {
    let mut pairs = [0; 26 * 26];

    for window in input.windows(2) {
        let key = encode(window[0], window[1]);
        pairs[key] += 1
    }

    pairs
}

fn step(pairs: &[usize], rules: &[usize]) -> [usize; 26 * 26] {
    let mut new_pairs = [0; 26 * 26];

    for (pair, &count) in pairs.iter().enumerate() {
        if count > 0 {
            let mid = rules[pair];
            let a = pair / 26;
            let b = pair % 26;

            new_pairs[a * 26 + mid] += count;
            new_pairs[mid * 26 + b] += count;
        }
    }

    new_pairs
}

fn solve<const N: usize>(input: &[u8], rules: &[usize; 26 * 26]) -> usize {
    let mut counts = [0usize; 26];
    counts[ord(input[0])] = 1;

    (0..N)
        .fold(pairs(input), |pairs, _| step(&pairs, rules))
        .into_iter()
        .enumerate()
        .for_each(|(pair, count)| {
            counts[pair % 26] += count;
        });

    counts.iter().max().unwrap() - counts.iter().filter(|&&count| count > 0).min().unwrap()
}

pub fn part1(input: &(Vec<u8>, [usize; 26 * 26])) -> usize {
    solve::<10>(&input.0, &input.1)
}

pub fn part2(input: &(Vec<u8>, [usize; 26 * 26])) -> usize {
    solve::<40>(&input.0, &input.1)
}
