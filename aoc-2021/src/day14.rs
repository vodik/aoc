use nom::{
    bytes::complete::{tag, take_while},
    combinator::{all_consuming, map, opt},
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

fn index(c: u8) -> usize {
    (c - b'A') as usize
}

fn index_pair(a: u8, b: u8) -> usize {
    index(a) * 26 + index(b)
}

fn word(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        take_while(move |c: char| c.is_ascii_uppercase()),
        |word: &str| word.bytes().collect(),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (usize, usize, usize)> {
    map(separated_pair(word, tag(" -> "), word), |(pair, result)| {
        let a = index(pair[0]);
        let b = index(pair[1]);
        let c = index(result[0]);
        (a * 26 + b, a * 26 + c, c * 26 + b)
    })(input)
}

fn parse_template(input: &str) -> IResult<&str, Vec<u8>> {
    word(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    separated_list1(tag("\n"), parse_rule)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<u8>, Vec<(usize, usize, usize)>)> {
    terminated(
        separated_pair(parse_template, tag("\n\n"), parse_rules),
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

struct Polymer<'a> {
    pairs: Vec<usize>,
    rules: &'a [(usize, usize, usize)],
}

impl<'a> Polymer<'a> {
    fn new(template: &[u8], rules: &'a [(usize, usize, usize)]) -> Self {
        let mut pairs = vec![0; 26 * 26];

        for window in template.windows(2) {
            let key = index_pair(window[0], window[1]);
            pairs[key] += 1
        }

        Self { pairs, rules }
    }

    fn step(&mut self) {
        let mut new_pairs = vec![0; 26 * 26];

        for &(index, p1, p2) in self.rules {
            let count = self.pairs[index];
            new_pairs[p1] += count;
            new_pairs[p2] += count;
        }

        self.pairs = new_pairs;
    }

    fn iter(&self) -> impl Iterator<Item = &usize> + '_ {
        self.pairs.iter()
    }
}

fn solve<const N: usize>(template: &[u8], rules: &[(usize, usize, usize)]) -> usize {
    let mut polymer = Polymer::new(template, rules);
    for _ in 0..N {
        polymer.step();
    }

    let mut counts = [0usize; 26];
    for (pair, count) in polymer.iter().enumerate() {
        counts[pair % 26] += count;
    }
    counts[index(template[0])] = 1;

    let max = counts.iter().max().unwrap();
    let min = counts.iter().filter(|&&count| count > 0).min().unwrap();
    max - min
}

pub fn part1(input: &(Vec<u8>, Vec<(usize, usize, usize)>)) -> usize {
    solve::<10>(&input.0, &input.1)
}

pub fn part2(input: &(Vec<u8>, Vec<(usize, usize, usize)>)) -> usize {
    solve::<40>(&input.0, &input.1)
}
