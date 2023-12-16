use std::{
    mem::{self, MaybeUninit},
    ptr,
};

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_end()
        .split(',')
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

fn hash_and_label(input: &[u8]) -> (u8, u32) {
    input.iter().fold((0u8, 0u32), |(hash, label), &c| {
        (
            hash.wrapping_add(c).wrapping_mul(17),
            (label << 8) | c as u32,
        )
    })
}

#[derive(Debug, Default, Clone)]
struct Entry {
    lenses: [Option<(u32, u8)>; 6],
    len: usize,
}

impl Entry {
    fn remove(&mut self, label: u32) {
        let position = self.iter().position(|entry| entry.0 == label);
        if let Some(position) = position {
            self.lenses.copy_within(position + 1.., position);
            self.len -= 1;
        }
    }

    fn insert(&mut self, label: u32, focal_length: u8) {
        if let Some(entry) = self.iter_mut().find(|entry| entry.0 == label) {
            entry.1 = focal_length;
            return;
        }
        self.lenses[self.len] = Some((label, focal_length));
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = &(u32, u8)> + '_ {
        self.lenses.iter().take(self.len).flatten()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut (u32, u8)> + '_ {
        self.lenses.iter_mut().take(self.len).flatten()
    }
}

pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut boxes = stack_alloc_boxes();

    for step in input {
        let len = step.len();
        match step[len - 2..] {
            [_, b'-'] => {
                let (hash, label) = hash_and_label(&step[..len - 1]);
                boxes.get_mut(hash as usize).unwrap().remove(label);
            }
            [b'=', digit] => {
                let (hash, label) = hash_and_label(&step[..len - 2]);
                boxes
                    .get_mut(hash as usize)
                    .unwrap()
                    .insert(label, digit - b'0');
            }
            _ => {}
        }
    }

    boxes
        .iter()
        .zip(1..boxes.len() + 1)
        .map(|(entry, box_no)| {
            entry
                .iter()
                .zip(1..entry.len() + 1)
                .map(|(&(_, focal_length), lens_pos)| box_no * lens_pos * focal_length as usize)
                .sum::<usize>()
        })
        .sum()
}

fn stack_alloc_boxes() -> [Entry; 256] {
    let mut data: [MaybeUninit<Entry>; 256] = unsafe { MaybeUninit::uninit().assume_init() };
    data.iter_mut().for_each(|elem| unsafe {
        ptr::write(elem.as_mut_ptr(), Entry::default());
    });
    unsafe { mem::transmute::<_, [Entry; 256]>(data) }
}
