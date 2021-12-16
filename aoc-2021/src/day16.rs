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
pub enum PacketBody {
    Literal(u64),
    Nested(Vec<Packet>),
}

// pub enum Op {
//     Sum,
//     Product,
//     Min,
//     Max,
//     GreaterThan,
//     LessThan,
//     Equal
// }

#[derive(Debug)]
pub struct Packet {
    version: u32,
    typ: u32,
    body: PacketBody,
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

fn parse_value(reader: &mut Reader) -> Option<(PacketBody, usize)> {
    let mut read = 0;
    let mut value = 0;

    loop {
        let c = reader.read(1)?;
        value <<= 4;
        value |= reader.read(4)? as u64;
        read += 5;

        if c == 0 {
            break Some((PacketBody::Literal(value), read));
        }
    }
}

fn parse_operator(reader: &mut Reader) -> Option<(PacketBody, usize)> {
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

        Some((PacketBody::Nested(packets), 16 + length))
    } else {
        let chunks = reader.read(11)?;
        let mut read = 12;
        let mut packets = Vec::with_capacity(chunks as usize);

        for _ in 0..chunks {
            let (packet, r) = parse_packet(reader).unwrap();
            read += r;
            packets.push(packet);
        }

        Some((PacketBody::Nested(packets), read))
    }
}

fn parse_packet(reader: &mut Reader) -> Option<(Packet, usize)> {
    let version = reader.read(3)?;
    let typ = reader.read(3)?;
    let mut read = 6;

    let (body, r) = match typ {
        4 => parse_value(reader),
        _ => parse_operator(reader),
    }?;
    read += r;

    Some((Packet { version, typ, body }, read))
}

fn versions(packet: &Packet) -> u32 {
    packet.version
        + match &packet.body {
            PacketBody::Literal(_) => 0,
            PacketBody::Nested(nested) => nested.iter().map(versions).sum(),
        }
}

fn run(packet: &Packet) -> u64 {
    match (packet.typ, &packet.body) {
        (0, PacketBody::Nested(nested)) => nested.iter().map(run).sum(),
        (1, PacketBody::Nested(nested)) => nested.iter().map(run).product(),
        (2, PacketBody::Nested(nested)) => nested.iter().map(run).min().unwrap(),
        (3, PacketBody::Nested(nested)) => nested.iter().map(run).max().unwrap(),
        (4, PacketBody::Literal(v)) => *v,
        (5, PacketBody::Nested(nested)) => {
            if run(&nested[0]) > run(&nested[1]) {
                1
            } else {
                0
            }
        }
        (6, PacketBody::Nested(nested)) => {
            if run(&nested[0]) < run(&nested[1]) {
                1
            } else {
                0
            }
        }
        (7, PacketBody::Nested(nested)) => {
            if run(&nested[0]) == run(&nested[1]) {
                1
            } else {
                0
            }
        }
        (x, _) => unimplemented!("{}", x),
    }
}

pub fn part1(input: &Packet) -> u32 {
    versions(input)
}

pub fn part2(input: &Packet) -> u64 {
    run(input)
}
