use std::num::NonZeroU16;

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim_end()
        .split(',')
        .map(|s| s.bytes().collect::<Vec<_>>())
        .collect()
}

#[inline(always)]
fn hash_and_label(input: &[u8]) -> (u8, u16) {
    input.iter().fold((0u8, 0u16), |(hash, label), &c| {
        (
            hash.wrapping_add(c).wrapping_mul(17),
            (label << 8) | c as u16,
        )
    })
}

pub fn part1(input: &[Vec<u8>]) -> u32 {
    input.iter().map(|step| hash_and_label(step).0 as u32).sum()
}

#[derive(Debug, Default, Clone, Copy)]
struct Entry {
    lenses: [Option<(NonZeroU16, u8)>; 6],
    len: usize,
}

impl Entry {
    fn insert(&mut self, label: u16, focal_length: u8) {
        let label = NonZeroU16::new(label).unwrap();
        if let Some(entry) = self.iter_mut().find(|entry| entry.0 == label) {
            entry.1 = focal_length;
            return;
        }
        self.lenses[self.len] = Some((label, focal_length));
        self.len += 1;
    }

    fn remove(&mut self, label: u16) {
        let label = NonZeroU16::new(label).unwrap();
        let position = self.iter().position(|entry| entry.0 == label);
        if let Some(position) = position {
            self.lenses.copy_within(position + 1.., position);
            self.len -= 1;
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = &(NonZeroU16, u8)> + '_ {
        self.lenses.iter().take(self.len).flatten()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut (NonZeroU16, u8)> + '_ {
        self.lenses.iter_mut().take(self.len).flatten()
    }
}

pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut boxes = [Entry::default(); 256];

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
