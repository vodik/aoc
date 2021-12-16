struct Reader<'a> {
    input: &'a [u8],
    pos: usize,
    bit: usize,
}

impl<'a> Reader<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            pos: 0,
            bit: 3,
        }
    }

    fn flush(&mut self) {
        self.pos += 1;
        self.bit = 3;
    }

    fn read_bit(&mut self) -> u8 {
        let mask = 1 << self.bit;
        let out = (self.input[self.pos] & mask) >> self.bit;

        if self.bit == 0 {
            self.flush();
        } else {
            self.bit -= 1;
        }

        out
    }

    fn read_byte(&mut self) -> u8 {
        let out = self.input[self.pos];
        self.flush();
        out
    }

    fn read(&mut self, size: usize) -> u32 {
        if size < 4 {
            self.read_small(size)
        } else {
            self.read_large(size)
        }
    }

    fn read_small(&mut self, size: usize) -> u32 {
        let mut out = 0;
        for _ in 0..size {
            out <<= 1;
            out |= self.read_bit() as u32
        }
        out
    }

    fn read_large(&mut self, mut size: usize) -> u32 {
        let mut out = 0;

        // read til 4 byte aligned
        match self.bit {
            0 => {
                out <<= 1;
                out |= self.input[self.pos] as u32 & 0b0001;
                size -= 1;
            }
            1 => {
                out <<= 2;
                out |= self.input[self.pos] as u32 & 0b0011;
                size -= 2;
            }
            2 => {
                out <<= 3;
                out |= self.input[self.pos] as u32 & 0b0111;
                size -= 3;
            }
            3 => {
                out <<= 4;
                out |= self.input[self.pos] as u32;
                size -= 4;
            }
            _ => unreachable!(),
        }
        self.flush();

        // read aligned
        for _ in 0..(size / 4) {
            out <<= 4;
            out |= self.read_byte() as u32;
        }

        // read leftover
        for _ in 0..(size % 4) {
            out <<= 1;
            out |= self.read_bit() as u32
        }

        out
    }
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

fn parse_literal(reader: &mut Reader) -> (Body, usize) {
    let mut read = 0;
    let mut value = 0;

    loop {
        let mark = reader.read(1);
        value <<= 4;
        value |= reader.read(4) as u64;
        read += 5;

        if mark == 0 {
            break (Body::Literal(value), read);
        }
    }
}

fn parse_form(op: Op, reader: &mut Reader) -> (Body, usize) {
    let length_encoding = reader.read(1);
    if length_encoding == 1 {
        let chunks = reader.read(11);
        let mut read = 0;
        let mut packets = Vec::with_capacity(chunks as usize);

        for _ in 0..chunks {
            let (packet, size) = parse_packet(reader);
            read += size;
            packets.push(packet);
        }

        (Body::Form(op, packets), 12 + read)
    } else {
        let length = reader.read(15) as usize;
        let mut read = 0usize;
        let mut packets = Vec::new();

        while read < length {
            let (packet, size) = parse_packet(reader);
            read += size;
            packets.push(packet);
        }

        (Body::Form(op, packets), 16 + read)
    }
}

fn parse_packet(reader: &mut Reader) -> (Packet, usize) {
    let version = reader.read(3);
    let op = reader.read(3);

    let mut read = 6;
    let (body, size) = match op {
        0 => parse_form(Op::Sum, reader),
        1 => parse_form(Op::Product, reader),
        2 => parse_form(Op::Min, reader),
        3 => parse_form(Op::Max, reader),
        4 => parse_literal(reader),
        5 => parse_form(Op::GreaterThan, reader),
        6 => parse_form(Op::LessThan, reader),
        7 => parse_form(Op::Equal, reader),
        _ => unreachable!(),
    };
    read += size;

    (Packet { version, body }, read)
}

impl Packet {
    fn parse(bytes: &[u8]) -> Self {
        let mut reader = Reader::new(bytes);
        let (packet, _) = parse_packet(&mut reader);
        packet
    }

    fn sum_versions(&self) -> u32 {
        self.version
            + match &self.body {
                Body::Literal(_) => 0,
                Body::Form(_, body) => body.iter().map(Packet::sum_versions).sum(),
            }
    }

    fn eval(&self) -> u64 {
        match &self.body {
            Body::Literal(value) => *value,
            Body::Form(Op::Sum, body) => body.iter().map(Packet::eval).sum(),
            Body::Form(Op::Product, body) => body.iter().map(Packet::eval).product(),
            Body::Form(Op::Min, body) => body.iter().map(Packet::eval).min().unwrap(),
            Body::Form(Op::Max, body) => body.iter().map(Packet::eval).max().unwrap(),
            Body::Form(Op::GreaterThan, body) => {
                if body[0].eval() > body[1].eval() {
                    1
                } else {
                    0
                }
            }
            Body::Form(Op::LessThan, body) => {
                if body[0].eval() < body[1].eval() {
                    1
                } else {
                    0
                }
            }
            Body::Form(Op::Equal, body) => {
                if body[0].eval() == body[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

pub fn parse_input(input: &str) -> Packet {
    let bytes: Vec<u8> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|b| u8::try_from(b.to_digit(16).unwrap()).unwrap())
        })
        .collect();

    Packet::parse(&bytes)
}

pub fn part1(packet: &Packet) -> u32 {
    packet.sum_versions()
}

pub fn part2(packet: &Packet) -> u64 {
    packet.eval()
}
