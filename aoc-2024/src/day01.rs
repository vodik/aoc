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

fn group_right(right: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut frequencies = Vec::with_capacity(right.len());
    for &value in right {
        match frequencies.last_mut() {
            Some((state, count)) if value == *state => *count += 1,
            _ => frequencies.push((value, 1)),
        }
    }
    frequencies
}

#[cfg(feature = "avx2")]
#[target_feature(enable = "avx2")]
unsafe fn part2_avx2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    use std::arch::x86_64::*;

    let frequencies = group_right(right);

    let mut acc = 0;
    let mut cursor = 0;
    for (right_value, frequency) in frequencies {
        let right_simd = _mm256_set1_epi32(right_value as i32);
        while cursor + 8 <= left.len() {
            let chunk = _mm256_loadu_si256(left[cursor..].as_ptr() as *const _);
            let mask = _mm256_movemask_epi8(_mm256_or_si256(
                _mm256_cmpgt_epi32(chunk, right_simd),
                _mm256_cmpeq_epi32(chunk, right_simd),
            ));
            if mask != 0 {
                cursor += mask.trailing_zeros() as usize / 4;
                break;
            }
            cursor += 8;
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

#[cfg(feature = "avx2")]
pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    unsafe { part2_avx2(input) }
}

#[cfg(not(feature = "avx2"))]
pub fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let frequencies = group_right(right);

    let mut acc = 0;
    let mut cursor = 0;
    for (right_value, frequency) in frequencies {
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
