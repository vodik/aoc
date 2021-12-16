use std::slice;

pub fn parse_input(input: &str) -> Packet {
    let bytes: Vec<u8> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|b| u8::try_from(b.to_digit(16).unwrap()).unwrap())
        })
        .collect();

    let mut reader = Reader::new(&bytes).unwrap();
    let (packet, _) = parse_packet(&mut reader).unwrap();
    packet
}

#[derive(Debug)]
pub enum Op {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug)]
pub enum Body {
    Literal(u64),
    Form(Op, Vec<Packet>),
}

#[derive(Debug)]
pub struct Packet {
    version: u32,
    body: Body,
}

struct Reader<'a> {
    curr: Option<u8>,
    bit: u8,
    iter: slice::Iter<'a, u8>,
}

impl<'a> Reader<'a> {
    fn new(input: &'a [u8]) -> Option<Self> {
        let mut iter = input.iter();
        Some(Self {
            curr: iter.next().copied(),
            bit: 3,
            iter,
        })
    }

    fn next(&mut self) -> Option<u8> {
        let mask = 1 << self.bit;
        let out = (self.curr? & mask) >> self.bit;

        if self.bit == 0 {
            self.bit = 3;
            self.curr = self.iter.next().copied();
        } else {
            self.bit -= 1;
        }

        Some(out)
    }

    fn read(&mut self, mut size: usize) -> Option<u32> {
        let mut out = 0;

        // to alignment
        if self.bit != 3 {
            let bits = usize::min(self.bit as usize + 1, size);
            size -= bits;

            for _ in 0..bits {
                out <<= 1;
                out |= self.next()? as u32
            }
        }

        // aligned
        for _ in 0..(size / 4) {
            out <<= 4;
            out |= self.curr? as u32;
            self.curr = self.iter.next().copied();
        }

        // leftover
        for _ in 0..(size % 4) {
            out <<= 1;
            out |= self.next()? as u32
        }

        Some(out)
    }
}

fn parse_literal(reader: &mut Reader) -> Option<(Body, usize)> {
    let mut read = 0;
    let mut value = 0;

    loop {
        let c = reader.read(1)?;
        value <<= 4;
        value |= reader.read(4)? as u64;
        read += 5;

        if c == 0 {
            break Some((Body::Literal(value), read));
        }
    }
}

fn parse_form(op: Op, reader: &mut Reader) -> Option<(Body, usize)> {
    let chunk_encoded = reader.read(1)?;
    if chunk_encoded == 0 {
        let length = reader.read(15)? as usize;
        let mut read = 0usize;
        let mut packets = Vec::new();

        while read < length {
            let (packet, r) = parse_packet(reader).unwrap();
            read += r;
            packets.push(packet);
        }

        Some((Body::Form(op, packets), 16 + length))
    } else {
        let chunks = reader.read(11)?;
        let mut read = 12;
        let mut packets = Vec::with_capacity(chunks as usize);

        for _ in 0..chunks {
            let (packet, r) = parse_packet(reader).unwrap();
            read += r;
            packets.push(packet);
        }

        Some((Body::Form(op, packets), read))
    }
}

fn parse_packet(reader: &mut Reader) -> Option<(Packet, usize)> {
    let version = reader.read(3)?;
    let op = reader.read(3)?;

    let mut read = 6;
    let (body, r) = match op {
        0 => parse_form(Op::Sum, reader),
        1 => parse_form(Op::Product, reader),
        2 => parse_form(Op::Min, reader),
        3 => parse_form(Op::Max, reader),
        4 => parse_literal(reader),
        5 => parse_form(Op::GreaterThan, reader),
        6 => parse_form(Op::LessThan, reader),
        7 => parse_form(Op::Equal, reader),
        _ => unreachable!(),
    }?;
    read += r;

    Some((Packet { version, body }, read))
}

fn versions(packet: &Packet) -> u32 {
    packet.version
        + match &packet.body {
            Body::Literal(_) => 0,
            Body::Form(_, nested) => nested.iter().map(versions).sum(),
        }
}

fn eval(packet: &Packet) -> u64 {
    match &packet.body {
        Body::Literal(value) => *value,
        Body::Form(Op::Sum, body) => body.iter().map(eval).sum(),
        Body::Form(Op::Product, body) => body.iter().map(eval).product(),
        Body::Form(Op::Min, body) => body.iter().map(eval).min().unwrap(),
        Body::Form(Op::Max, body) => body.iter().map(eval).max().unwrap(),
        Body::Form(Op::GreaterThan, body) => {
            if eval(&body[0]) > eval(&body[1]) {
                1
            } else {
                0
            }
        }
        Body::Form(Op::LessThan, body) => {
            if eval(&body[0]) < eval(&body[1]) {
                1
            } else {
                0
            }
        }
        Body::Form(Op::Equal, body) => {
            if eval(&body[0]) == eval(&body[1]) {
                1
            } else {
                0
            }
        }
    }
}

pub fn part1(input: &Packet) -> u32 {
    versions(input)
}

pub fn part2(input: &Packet) -> u64 {
    eval(input)
}
