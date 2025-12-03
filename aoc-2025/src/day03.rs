pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect()
}

pub fn select_batteries(bank: &[u8], keep: usize, stack: &mut Vec<u8>) -> u64 {
    let mut discards = bank.len() - keep;
    for &cell in bank {
        while discards > 0 {
            match stack.last() {
                Some(&last) if last < cell => {
                    stack.pop();
                    discards -= 1;
                }
                _ => break,
            }
        }
        stack.push(cell);
    }

    stack[..keep]
        .iter()
        .fold(0, |acc, &cell| acc * 10 + cell as u64)
}

pub fn part1(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .scan(Vec::with_capacity(100), |stack, bank| {
            stack.clear();
            Some(select_batteries(bank, 2, stack))
        })
        .sum()
}

pub fn part2(banks: &[Vec<u8>]) -> u64 {
    banks
        .iter()
        .scan(Vec::with_capacity(100), |stack, bank| {
            stack.clear();
            Some(select_batteries(bank, 12, stack))
        })
        .sum()
}
