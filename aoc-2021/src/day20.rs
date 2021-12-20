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

fn eval(point: usize, image: &[u8], width: usize, default: u8) -> u16 {
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
        .map(|p| (p.map(|p| image[p]).unwrap_or(default) as u16))
        .fold(0, |acc, b| acc << 1 | b)
}

fn step(image: &[u8], width: usize, algo: &[u8], flip: bool) -> (Vec<u8>, usize) {
    let default = if flip { 1 } else { 0 };
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
    for (pos, cell) in new_image.iter_mut().enumerate() {
        let score = eval(pos, &enlarged, new_width, default);
        *cell = algo[score as usize];
    }

    (new_image, new_width)
}

fn solve<const N: usize>(algorithm: &[u8], image: &[u8]) -> usize {
    let mut width = WIDTH;
    let mut image = image.to_vec();

    for generation in 0..N {
        let flip = algorithm[0] == 1 && generation % 2 != 0;
        (image, width) = step(&image, width, algorithm, flip);
    }

    image.iter().filter(|&x| *x == 1).count()
}

pub fn part1((algorithm, image): &(Vec<u8>, Vec<u8>)) -> usize {
    solve::<2>(algorithm, image)
}

pub fn part2((algorithm, image): &(Vec<u8>, Vec<u8>)) -> usize {
    solve::<50>(algorithm, image)
}
