const WIDTH: usize = 100;
const GROW: usize = 1;

pub fn parse_input(input: &str) -> (Vec<u8>, Vec<u8>) {
    let mut lines = input.lines();

    let algorithm: Vec<_> = lines
        .next()
        .unwrap()
        .bytes()
        .map(|c| match c {
            b'.' => 0,
            b'#' => 1,
            _ => unreachable!(),
        })
        .collect();

    lines.next().unwrap();

    let image: Vec<_> = lines
        .flat_map(|line| {
            line.bytes().map(|c| match c {
                b'.' => 0,
                b'#' => 1,
                _ => unreachable!(),
            })
        })
        .collect();

    (algorithm, image)
}

fn window_fresh(point: usize, image: &[u8], width: usize, default: u8) -> u16 {
    let window = [
        point
            .checked_sub(1)
            .filter(|p| p % width != width - 1)
            .and_then(|p| p.checked_sub(width)),
        point.checked_sub(width),
        point
            .checked_add(1)
            .filter(|p| p % width != 0)
            .and_then(|p| p.checked_sub(width)),
        point.checked_sub(1).filter(|p| p % width != width - 1),
        Some(point),
        point.checked_add(1).filter(|p| p % width != 0),
        point
            .checked_sub(1)
            .filter(|p| p % width != width - 1)
            .and_then(|p| p.checked_add(width))
            .filter(|&p| p < width * width),
        point.checked_add(width).filter(|&p| p < width * width),
        point
            .checked_add(1)
            .filter(|p| p % width != 0)
            .and_then(|p| p.checked_add(width))
            .filter(|&p| p < width * width),
    ];

    window
        .iter()
        .map(|point| point.map(|p| image[p]).unwrap_or(default) as u16)
        .fold(0, |acc, bit| acc << 1 | bit)
}

fn window(point: usize, prev: u16, image: &[u8], width: usize, default: u8) -> u16 {
    let newy = point % width == 0;
    if newy {
        window_fresh(point, image, width, default)
    } else {
        let newdata = [
            point
                .checked_add(1)
                .filter(|p| p % width != 0)
                .and_then(|p| p.checked_sub(width)),
            point.checked_add(1).filter(|p| p % width != 0),
            point
                .checked_add(1)
                .filter(|p| p % width != 0)
                .and_then(|p| p.checked_add(width))
                .filter(|&p| p < width * width),
        ];

        let mut a = (prev & 0b000000111) << 1 & 0b000000111;
        a |= newdata[2].map(|p| image[p]).unwrap_or(default) as u16;
        let mut b = (prev & 0b000111000) << 1 & 0b000111000;
        b |= (newdata[1].map(|p| image[p]).unwrap_or(default) as u16) << 3;
        let mut c = (prev & 0b111000000) << 1 & 0b111000000;
        c |= (newdata[0].map(|p| image[p]).unwrap_or(default) as u16) << 6;

        a | b | c
    }
}

fn step(image: &[u8], width: usize, algorithm: &[u8], generation: usize) -> (Vec<u8>, usize) {
    let default = if generation % 2 != 0 { algorithm[0] } else { 0 };

    let height = image.len() / width;
    let new_width = width + GROW * 2;
    let new_height = height + GROW * 2;
    let mut enlarged = vec![default; new_width * new_height];

    let offset = new_width * GROW + GROW;
    for (line, chunk) in image.chunks(width).enumerate() {
        let pos = line * new_width + offset;
        enlarged[pos..pos + width].copy_from_slice(chunk);
    }

    let mut new_image = enlarged.clone();
    let mut score = 0;
    for (pos, cell) in new_image.iter_mut().enumerate() {
        score = window(pos, score, &enlarged, new_width, default);
        *cell = algorithm[score as usize];
    }

    (new_image, new_width)
}

fn solve<const N: usize>(algorithm: &[u8], image: &[u8]) -> usize {
    let width = WIDTH;
    let image = image.to_vec();

    (0..N)
        .fold((image, width), |(image, width), generation| {
            step(&image, width, algorithm, generation)
        })
        .0
        .iter()
        .filter(|&&cell| cell == 1)
        .count()
}

pub fn part1((algorithm, image): &(Vec<u8>, Vec<u8>)) -> usize {
    solve::<2>(algorithm, image)
}

pub fn part2((algorithm, image): &(Vec<u8>, Vec<u8>)) -> usize {
    solve::<50>(algorithm, image)
}
