pub fn parse_input(input: &str) -> &[u8] {
    input.as_bytes()
}

fn find_packet<const N: usize>(data: &[u8]) -> Option<usize> {
    let buf: usize = data[..N].iter().fold(0, |acc, &b| acc ^ 1 << (b - b'a'));
    if buf.count_ones() as usize == N {
        return Some(N);
    }

    data.windows(N + 1)
        .scan(buf, |buf, w| {
            *buf ^= 1 << (w[N] - b'a');
            *buf ^= 1 << (w[0] - b'a');
            Some(buf.count_ones() as usize)
        })
        .position(|acc| acc == N)
        .map(|pos| pos + N + 1)
}

pub fn part1(input: &[u8]) -> usize {
    find_packet::<4>(input).unwrap()
}

pub fn part2(input: &[u8]) -> usize {
    find_packet::<14>(input).unwrap()
}
