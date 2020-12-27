use std::collections::HashMap;
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

#[aoc(day23, part2)]
fn part2(data: &[u32]) -> u64 {
    const MAX_CUP: u32 = 1_000_000;

    let cups: Vec<u32> = data.iter().map(|&c| c as u32).chain(10..=MAX_CUP).collect();

    let map: HashMap<u32, usize> = cups
        .iter()
        .enumerate()
        .map(|(idx, cup)| (*cup, idx))
        .collect();

    let mut chains: Vec<usize> = cups
        .iter()
        .enumerate()
        .map(|(idx, _)| (idx + 1) % cups.len())
        .collect();

    let mut position = 0;
    for _ in 0..10_000_000 {
        let cup1_addr = chains[position];
        let cup2_addr = chains[cup1_addr];
        let cup3_addr = chains[cup2_addr];

        let next = [cups[cup1_addr], cups[cup2_addr], cups[cup3_addr]];
        let mut pivot = prev_cup(cups[position], MAX_CUP);
        while next.iter().any(|&c| c == pivot) {
            pivot = prev_cup(pivot, MAX_CUP);
        }

        let pivot_addr = map[&pivot];
        let next_addr = chains[cup3_addr];

        chains[position] = next_addr;
        chains[cup3_addr] = chains[pivot_addr];
        chains[pivot_addr] = cup1_addr;

        position = next_addr;
    }

    let one_addr = map[&1];
    let first_addr = chains[one_addr];
    let second_addr = chains[first_addr];

    cups[first_addr] as u64 * cups[second_addr] as u64
}
