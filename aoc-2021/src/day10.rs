pub fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

enum ParseResult {
    Ok,
    Incomplete(Vec<u8>),
    UnexpectedToken(u8),
}

fn parse_expr(expr: &[u8]) -> ParseResult {
    let mut stack = Vec::new();
    for &c in expr {
        match c {
            b'(' | b'[' | b'{' | b'<' => stack.push(c),
            b')' | b']' | b'}' | b'>' => match (stack.last(), c) {
                (Some(b'('), b')')
                | (Some(b'['), b']')
                | (Some(b'{'), b'}')
                | (Some(b'<'), b'>') => {
                    stack.pop();
                }
                (_, c) => {
                    return ParseResult::UnexpectedToken(c);
                }
            },
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        ParseResult::Ok
    } else {
        ParseResult::Incomplete(stack)
    }
}

pub fn part1(input: &[&[u8]]) -> usize {
    input
        .iter()
        .map(|expr| parse_expr(expr))
        .filter_map(|result| match result {
            ParseResult::UnexpectedToken(b')') => Some(3),
            ParseResult::UnexpectedToken(b']') => Some(57),
            ParseResult::UnexpectedToken(b'}') => Some(1197),
            ParseResult::UnexpectedToken(b'>') => Some(25137),
            _ => None,
        })
        .sum()
}

pub fn part2(input: &[&[u8]]) -> u64 {
    let mut count: Vec<u64> = input
        .iter()
        .map(|expr| parse_expr(expr))
        .filter_map(|result| match result {
            ParseResult::Incomplete(stack) => Some(stack),
            _ => None,
        })
        .map(|stack| {
            stack.into_iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();

    count.sort_unstable();
    count[count.len() / 2]
}
