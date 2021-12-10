pub fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

#[derive(Debug)]
enum Expr {
    Corrupt(u8),
    Incomplete(Vec<u8>),
}

impl Expr {
    fn parse(expr: &[u8]) -> Self {
        let mut stack = Vec::with_capacity(100);
        for &token in expr {
            match token {
                b'(' | b'[' | b'{' | b'<' => stack.push(token),
                b')' | b']' | b'}' | b'>' => match (stack.pop(), token) {
                    (Some(b'('), b')')
                    | (Some(b'['), b']')
                    | (Some(b'{'), b'}')
                    | (Some(b'<'), b'>') => {}
                    _ => return Expr::Corrupt(token),
                },
                _ => unreachable!(),
            }
        }
        Expr::Incomplete(stack)
    }
}

pub fn part1(input: &[&[u8]]) -> usize {
    input
        .iter()
        .filter_map(|expr| match Expr::parse(expr) {
            Expr::Corrupt(b')') => Some(3),
            Expr::Corrupt(b']') => Some(57),
            Expr::Corrupt(b'}') => Some(1197),
            Expr::Corrupt(b'>') => Some(25137),
            _ => None,
        })
        .sum()
}

pub fn part2(input: &[&[u8]]) -> u64 {
    let mut count: Vec<u64> = input
        .iter()
        .filter_map(|expr| match Expr::parse(expr) {
            Expr::Incomplete(stack) => Some(stack),
            _ => None,
        })
        .map(|stack| {
            stack
                .into_iter()
                .rev()
                .map(|c| match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect();

    count.sort_unstable();
    count[count.len() / 2]
}
