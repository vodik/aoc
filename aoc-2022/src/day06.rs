pub fn parse_input(input: &str) -> &[u8] {
    input.as_bytes()
}

fn ascii_alpha_bitmap(buf: &[u8]) -> u32 {
    buf.iter().fold(0u32, |acc, &b| acc | 1 << (b - b'a'))
}

fn find_packet<const N: usize>(buf: &[u8]) -> Option<usize> {
    buf.windows(N as usize)
        .position(|b| ascii_alpha_bitmap(b).count_ones() == N as u32)
        .map(|start| start + N)
}

pub fn part1(input: &[u8]) -> usize {
    find_packet::<4>(input).unwrap()
}

pub fn part2(input: &[u8]) -> usize {
    find_packet::<14>(input).unwrap()
}
