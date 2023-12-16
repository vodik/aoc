pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_end()
        .split(",")
        .map(|s| s.bytes().collect::<Vec<_>>())
        .collect()
}

fn hash(input: &[u8]) -> u8 {
    input
        .iter()
        .fold(0u8, |acc, &c| acc.wrapping_add(c).wrapping_mul(17))
}

pub fn part1(input: &[Vec<u8>]) -> u32 {
    input.iter().map(|step| hash(step) as u32).sum()
}

pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut boxes = vec![vec![]; 256];

    for step in input {
        let len = step.len();
        match step[len - 2..] {
            [_, b'-'] => {
                let label = &step[..len - 1];

                boxes
                    .get_mut(hash(label) as usize)
                    .unwrap()
                    .retain(|&(key, _)| key != label);
            }
            [b'=', digit] => {
                let label = &step[..len - 2];
                let focal_length = (digit as char).to_digit(10).unwrap() as usize;

                let entry = boxes.get_mut(hash(label) as usize).unwrap();
                match entry.iter_mut().find(|(key, _)| key == &label) {
                    Some(slot) => {
                        (*slot).1 = focal_length;
                    }
                    None => {
                        entry.push((label, focal_length));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_no, entry)| {
            entry
                .iter()
                .enumerate()
                .map(|(slot_no, &(_, focal_length))| (box_no + 1) * (slot_no + 1) * focal_length)
                .sum::<usize>()
        })
        .sum()
}

fn fun_name<'a>() -> [Vec<(&'a [u8], usize)>; 256] {
    let mut data: [std::mem::MaybeUninit<Vec<(&[u8], usize)>>; 256] =
        unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    for elem in &mut data[..] {
        unsafe {
            std::ptr::write(elem.as_mut_ptr(), Vec::with_capacity(4));
        }
    }

    unsafe { std::mem::transmute::<_, [Vec<(&[u8], usize)>; 256]>(data) }
}
