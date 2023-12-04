#[derive(Debug, Clone, Copy)]
pub enum Token {
    Digit(u32),
    Word(u32),
}

impl Token {
    fn digit(&self) -> Option<u32> {
        match self {
            Token::Digit(digit) => Some(*digit),
            Token::Word(_) => None,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Token::Digit(digit) => *digit,
            Token::Word(digit) => *digit,
        }
    }
}

fn sub_slices<T>(buf: &[T], len: usize) -> impl Iterator<Item = &[T]> + '_ {
    let max_len = buf.len();
    (0..max_len).map(move |index| &buf[index..(index + len).min(max_len)])
}

fn parse_line(bytes: &[u8]) -> Vec<Token> {
    sub_slices(bytes, 5)
        .filter_map(|window| {
            let first = window[0] as char;
            match first.to_digit(10) {
                Some(digit) => Some(Token::Digit(digit)),
                _ if window.starts_with(b"one") => Some(Token::Word(1)),
                _ if window.starts_with(b"two") => Some(Token::Word(2)),
                _ if window.starts_with(b"three") => Some(Token::Word(3)),
                _ if window.starts_with(b"four") => Some(Token::Word(4)),
                _ if window.starts_with(b"five") => Some(Token::Word(5)),
                _ if window.starts_with(b"six") => Some(Token::Word(6)),
                _ if window.starts_with(b"seven") => Some(Token::Word(7)),
                _ if window.starts_with(b"eight") => Some(Token::Word(8)),
                _ if window.starts_with(b"nine") => Some(Token::Word(9)),
                _ => None,
            }
        })
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Vec<Token>> {
    let mut buffer = Vec::with_capacity(1 << 7);
    input
        .lines()
        .map(|line| {
            buffer.clear();
            buffer.extend(line.bytes());
            parse_line(&buffer)
        })
        .collect()
}

pub fn part1(input: &[Vec<Token>]) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut it = line.iter().filter_map(Token::digit);
            let first = it.next().unwrap_or_default();
            first * 10 + it.last().unwrap_or(first)
        })
        .sum()
}

pub fn part2(input: &[Vec<Token>]) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut it = line.iter().map(Token::value);
            let first = it.next().unwrap_or_default();
            first * 10 + it.last().unwrap_or(first)
        })
        .sum()
}
