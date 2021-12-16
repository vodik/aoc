use std::slice;

pub fn parse_input(input: &str) -> Packet {
    let bytes: Vec<u8> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|b| u8::try_from(b.to_digit(16).unwrap()).unwrap())
        })
        .flat_map(|b| {
            [
                (b & 0b1000) >> 3,
                (b & 0b0100) >> 2,
                (b & 0b0010) >> 1,
                b & 0b0001,
            ]
        })
        .collect();

    let mut reader = Reader(bytes.iter());
    let (packet, _) = parse_packet(&mut reader).unwrap();
    packet
}

#[derive(Debug)]
pub enum PacketBody {
    Literal(u64),
    Nested(Vec<Packet>),
}

#[derive(Debug)]
pub struct Packet {
    version: u32,
    typ: u32,
    body: PacketBody,
}

struct Reader<'a>(slice::Iter<'a, u8>);

impl Reader<'_> {
    fn read(&mut self, size: usize) -> Option<u32> {
        let mut v = 0;
        for _ in 0..size {
            v <<= 1;
            v |= *self.0.next()? as u32
        }
        Some(v)
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
        let read = 16 + length;
        let mut r2 = 0usize;
        let mut packets = Vec::new();

        while r2 < length {
            let (packet, r) = parse_packet(reader).unwrap();
            r2 += r;
            packets.push(packet);
        }

        Some((PacketBody::Nested(packets), read))
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
