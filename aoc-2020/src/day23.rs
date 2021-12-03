use std::num::ParseIntError;

fn prev_cup(cup: u32, max: u32) -> u32 {
    if cup == 1 {
        max
    } else {
        cup - 1
    }
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.chars().map(|c| c.to_string().parse()).collect()
}

#[aoc(day23, part1)]
fn part1(data: &[u32]) -> String {
    let mut turn: Vec<u32> = data.to_vec();

    for _ in 0..100 {
        let start = turn[0];
        let next = &turn[1..=3];

        let mut pivot = prev_cup(start, 9);
        while next.iter().any(|&c| c == pivot) {
            pivot = prev_cup(pivot, 9);
        }

        let mut next_turn = Vec::with_capacity(9);
        for &cup in &turn[4..] {
            next_turn.push(cup);
            if cup == pivot {
                next_turn.extend(next);
            }
        }
        next_turn.push(start);

        turn = next_turn;
    }

    turn.split(|&cup| cup == 1)
        .rev()
        .flat_map(|chunk| chunk.iter().map(ToString::to_string))
        .collect::<Vec<_>>()
        .join("")
}

fn addr_to_cup(addr: usize) -> u32 {
    addr as u32 + 1
}

fn cup_to_addr(cup: u32) -> usize {
    (cup - 1) as usize
}

#[aoc(day23, part2)]
fn part2(data: &[u32]) -> u64 {
    const MAX_CUP: u32 = 1_000_000;

    let start_addr = cup_to_addr(*data.first().unwrap());

    let mut cup_links = data
        .windows(2)
        .map(|window| (window[0], cup_to_addr(window[1])))
        .collect::<Vec<_>>();
    cup_links.push((*data.last().unwrap(), data.len()));
    cup_links.sort_by(|(cup1, _), (cup2, _)| cup1.cmp(&cup2));

    let mut links: Vec<usize> = Vec::with_capacity(MAX_CUP as usize);
    links.extend(cup_links.into_iter().map(|(_, addr)| addr));
    links.extend(data.len() + 1..MAX_CUP as usize);
    links.push(start_addr);

    let mut position = start_addr;
    for _ in 0..10_000_000 {
        let cup1_addr = links[position];
        let cup2_addr = links[cup1_addr];
        let cup3_addr = links[cup2_addr];

        let next = [
            addr_to_cup(cup1_addr),
            addr_to_cup(cup2_addr),
            addr_to_cup(cup3_addr),
        ];
        let mut pivot = prev_cup(addr_to_cup(position), MAX_CUP);
        while next.iter().any(|&c| c == pivot) {
            pivot = prev_cup(pivot, MAX_CUP);
        }

        let pivot_addr = cup_to_addr(pivot);
        let next_addr = links[cup3_addr];

        links[position] = next_addr;
        links[cup3_addr] = links[pivot_addr];
        links[pivot_addr] = cup1_addr;

        position = next_addr;
    }

    let first_addr = cup_to_addr(1);
    let second_addr = links[first_addr];
    addr_to_cup(first_addr) as u64 * addr_to_cup(second_addr) as u64
}
