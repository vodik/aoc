use std::simd::{cmp::SimdPartialOrd, u32x8};

pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        left_list.push(left.parse().unwrap());
        right_list.push(right.parse().unwrap());
    }

    left_list.sort_unstable();
    right_list.sort_unstable();
    (left_list, right_list)
}

pub fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut frequencies = Vec::with_capacity(right.len());
    for &value in right {
        match frequencies.last_mut() {
            Some((state, count)) if value == *state => *count += 1,
            _ => frequencies.push((value, 1)),
        }
    }

    let mut acc = 0;
    let mut cursor = 0;
    for (right_value, frequency) in frequencies {
        let right_simd = u32x8::splat(right_value as u32);
        while cursor + u32x8::LEN <= left.len() {
            let chunk = u32x8::from_slice(&left[cursor..]);
            let mask = chunk.simd_ge(right_simd).to_bitmask();
            if mask != 0 {
                cursor += mask.trailing_zeros() as usize;
                break;
            }
            cursor += u32x8::LEN;
        }

        while cursor < left.len() && left[cursor] < right_value {
            cursor += 1;
        }

        if cursor < left.len() && left[cursor] == right_value {
            acc += right_value * frequency;
            cursor += 1;
        }

        if cursor >= left.len() {
            break;
        }
    }
    acc
}
