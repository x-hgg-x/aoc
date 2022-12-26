use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;

const BYTE: usize = 8;

enum PacketData {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Greater(Vec<Packet>),
    Less(Vec<Packet>),
    Equal(Vec<Packet>),
}

struct Packet {
    version: u64,
    data: PacketData,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        fn sum(packet: &Packet) -> u64 {
            let sum = match &packet.data {
                PacketData::Sum(sub_packets)
                | PacketData::Product(sub_packets)
                | PacketData::Min(sub_packets)
                | PacketData::Max(sub_packets)
                | PacketData::Greater(sub_packets)
                | PacketData::Less(sub_packets)
                | PacketData::Equal(sub_packets) => sub_packets.iter().map(sum).sum(),
                _ => 0,
            };
            sum + packet.version
        }
        sum(self)
    }

    fn value(&self) -> u64 {
        match &self.data {
            PacketData::Literal(value) => *value,
            PacketData::Sum(sub_packets) => sub_packets.iter().map(Self::value).sum(),
            PacketData::Product(sub_packets) => sub_packets.iter().map(Self::value).product(),
            PacketData::Min(sub_packets) => sub_packets.iter().map(Self::value).min().unwrap_or_default(),
            PacketData::Max(sub_packets) => sub_packets.iter().map(Self::value).max().unwrap_or_default(),
            PacketData::Greater(sub_packets) => (sub_packets[0].value() > sub_packets[1].value()) as u64,
            PacketData::Less(sub_packets) => (sub_packets[0].value() < sub_packets[1].value()) as u64,
            PacketData::Equal(sub_packets) => (sub_packets[0].value() == sub_packets[1].value()) as u64,
        }
    }
}

struct BitCursor<'a> {
    remaining: &'a [u8],
    offset: usize,
    bit_count: usize,
}

impl<'a> BitCursor<'a> {
    fn new(remaining: &'a [u8]) -> Self {
        Self { remaining, offset: 0, bit_count: 0 }
    }

    fn read_bits(&mut self, count: usize) -> Result<u64> {
        if count == 0 {
            return Ok(0);
        }

        ensure!(count <= 64, "cannot read more than 64 bits");
        ensure!(!self.remaining.is_empty(), "no remaining data");

        let byte_count = (self.offset + count) / BYTE;
        let new_offset = (self.offset + count) % BYTE;

        ensure!(self.remaining.len() >= byte_count, "out of bound read");

        let first = self.remaining[0];
        let (bytes, remaining) = self.remaining.split_at(byte_count);

        let first_n = BYTE.min(self.offset + count);
        let mut value = Self::bits(first, self.offset, first_n) as u64;

        for &byte in bytes.iter().skip(1) {
            value <<= 8;
            value += byte as u64;
        }

        if byte_count > 0 && new_offset > 0 {
            ensure!(!remaining.is_empty(), "no remaining data");
            value <<= new_offset;
            value += Self::bits(remaining[0], 0, new_offset) as u64;
        }

        self.remaining = remaining;
        self.offset = new_offset;
        self.bit_count += count;

        Ok(value)
    }

    fn bits(value: u8, offset_before: usize, offset_after: usize) -> u8 {
        let before = BYTE - offset_before;
        let after = BYTE - offset_after;
        let mask = (((1 << before) - 1) & !((1 << after) - 1)) as u8;
        (value & mask) >> after
    }

    fn parse_packet(&mut self) -> Result<Packet> {
        let version = self.read_bits(3)?;
        let packet_type = self.read_bits(3)?;

        match packet_type {
            0 => Ok(Packet { version, data: PacketData::Sum(self.parse_sub_packets()?) }),
            1 => Ok(Packet { version, data: PacketData::Product(self.parse_sub_packets()?) }),
            2 => Ok(Packet { version, data: PacketData::Min(self.parse_sub_packets()?) }),
            3 => Ok(Packet { version, data: PacketData::Max(self.parse_sub_packets()?) }),
            4 => Ok(Packet { version, data: PacketData::Literal(self.parse_literal()?) }),
            5 => Ok(Packet { version, data: PacketData::Greater(self.parse_sub_packets()?) }),
            6 => Ok(Packet { version, data: PacketData::Less(self.parse_sub_packets()?) }),
            7 => Ok(Packet { version, data: PacketData::Equal(self.parse_sub_packets()?) }),
            _ => bail!("unknown packet type: {packet_type}"),
        }
    }

    fn parse_literal(&mut self) -> Result<u64> {
        let mut value = 0;

        loop {
            let is_last = self.read_bits(1)? == 0;

            value <<= 4;
            value += self.read_bits(4)?;

            if is_last {
                break Ok(value);
            }
        }
    }

    fn parse_sub_packets(&mut self) -> Result<Vec<Packet>> {
        let (max_bits, max_sub_packets) = match self.read_bits(1)? {
            0 => (self.read_bits(15)? as usize, usize::MAX),
            1 => (usize::MAX, self.read_bits(11)? as usize),
            other => bail!("unknown packet length type: {other}"),
        };

        let start_bit_count = self.bit_count;
        let mut sub_packets = Vec::new();

        while self.bit_count - start_bit_count < max_bits && sub_packets.len() < max_sub_packets {
            sub_packets.push(self.parse_packet()?);
        }

        Ok(sub_packets)
    }
}

fn parse_hex(x: u8) -> Result<u8> {
    match x {
        b'0'..=b'9' => Ok(x - b'0'),
        b'A'..=b'F' => Ok(x - b'A' + 10),
        b'a'..=b'f' => Ok(x - b'a' + 10),
        _ => bail!("invalid hex char"),
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim();

    ensure!(input.len() % 2 == 0, "input must have an even number of bytes");

    let bytes: Vec<_> = input
        .as_bytes()
        .chunks_exact(2)
        .map(|x| {
            let x0 = parse_hex(x[0])?;
            let x1 = parse_hex(x[1])?;
            Result::Ok((x0 << 4) + x1)
        })
        .try_collect()?;

    let packet = BitCursor::new(&bytes).parse_packet()?;

    let result1 = packet.version_sum();
    let result2 = packet.value();

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
