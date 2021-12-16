use anyhow::Result;

#[inline]
fn to_binary_chars(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PacketKind {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    version: u8,
    kind: PacketKind,
}

impl Packet {
    fn from_input(s: &str) -> Self {
        let s: String = s.chars().map(to_binary_chars).collect();
        let (s, _) = Self::parse(s.as_str()).unwrap();
        s
    }

    fn parse_multiple(s: &str) -> Option<(Vec<Self>, usize)> {
        let length_type = s.as_bytes()[0];
        let mut current = 1;
        if length_type == b'0' {
            let sub = s.get(current..current + 15)?;
            current += 15;
            let length = usize::from_str_radix(sub, 2).ok()?;
            let expected = current + length;
            let mut packets = Vec::new();
            while current != expected {
                let (packet, len) = Self::parse(&s[current..])?;
                packets.push(packet);
                current += len;
            }
            return Some((packets, current));
        } else if length_type == b'1' {
            let sub = s.get(current..current + 11)?;
            current += 11;
            let count = usize::from_str_radix(sub, 2).ok()?;
            let mut packets = Vec::with_capacity(count);
            while packets.len() != count {
                let (packet, len) = Self::parse(&s[current..])?;
                packets.push(packet);
                current += len;
            }
            return Some((packets, current));
        } else {
            None
        }
    }

    fn parse(s: &str) -> Option<(Self, usize)> {
        // Read the version header
        let version = u8::from_str_radix(s.get(0..3)?, 2).ok()?;
        let t = u8::from_str_radix(s.get(3..6)?, 2).ok()?;
        let mut current = 6;
        if t == 4 {
            let mut buffer = String::with_capacity(64);
            for chunk in s[current..].as_bytes().chunks(5) {
                for ch in &chunk[1..] {
                    buffer.push(*ch as char);
                }
                current += chunk.len();
                if chunk[0] == b'0' {
                    break;
                }
            }
            let kind = PacketKind::Literal(u64::from_str_radix(buffer.as_str(), 2).ok()?);
            Some((Packet { version, kind }, current))
        } else {
            let (packets, length) = Self::parse_multiple(&s[current..])?;
            current += length;
            match t {
                0 => Some((
                    Packet {
                        version,
                        kind: PacketKind::Sum(packets),
                    },
                    current,
                )),
                1 => Some((
                    Packet {
                        version,
                        kind: PacketKind::Product(packets),
                    },
                    current,
                )),
                2 => Some((
                    Packet {
                        version,
                        kind: PacketKind::Minimum(packets),
                    },
                    current,
                )),
                3 => Some((
                    Packet {
                        version,
                        kind: PacketKind::Maximum(packets),
                    },
                    current,
                )),
                5 => Some((
                    Packet {
                        version,
                        kind: PacketKind::GreaterThan(packets),
                    },
                    current,
                )),
                6 => Some((
                    Packet {
                        version,
                        kind: PacketKind::LessThan(packets),
                    },
                    current,
                )),
                7 => Some((
                    Packet {
                        version,
                        kind: PacketKind::EqualTo(packets),
                    },
                    current,
                )),
                _ => None,
            }
        }
    }

    fn version_sum(&self) -> usize {
        match &self.kind {
            PacketKind::Literal(_) => self.version as usize,
            PacketKind::Sum(rest)
            | PacketKind::Product(rest)
            | PacketKind::Minimum(rest)
            | PacketKind::Maximum(rest)
            | PacketKind::GreaterThan(rest)
            | PacketKind::LessThan(rest)
            | PacketKind::EqualTo(rest) => {
                self.version as usize + rest.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }

    fn evaluate(&self) -> usize {
        match &self.kind {
            PacketKind::Literal(x) => *x as usize,
            PacketKind::Sum(rest) => rest.iter().map(|p| p.evaluate()).sum::<usize>(),
            PacketKind::Product(rest) => rest.iter().fold(1, |a, b| a * b.evaluate()),
            PacketKind::Minimum(rest) => rest.iter().map(|p| p.evaluate()).min().unwrap(),
            PacketKind::Maximum(rest) => rest.iter().map(|p| p.evaluate()).max().unwrap(),
            PacketKind::GreaterThan(packets) => {
                let lhs = &packets[0];
                let rhs = &packets[1];
                if lhs.evaluate() > rhs.evaluate() {
                    1
                } else {
                    0
                }
            }
            PacketKind::LessThan(packets) => {
                let lhs = &packets[0];
                let rhs = &packets[1];
                if lhs.evaluate() < rhs.evaluate() {
                    1
                } else {
                    0
                }
            }
            PacketKind::EqualTo(packets) => {
                let lhs = &packets[0];
                let rhs = &packets[1];
                if lhs.evaluate() == rhs.evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let packet = Packet::from_input(input);
    packet.version_sum()
}

fn part2(input: &str) -> usize {
    let packet = Packet::from_input(input);
    packet.evaluate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_value() {
        let input = "D2FE28";
        let packet = Packet::from_input(input);
        assert!(matches!(packet.kind, PacketKind::Literal(2021)))
    }

    #[test]
    fn test_length_type_zero() {
        let input = "38006F45291200";
        let packet = Packet::from_input(input);
        assert_eq!(packet.version, 1);
        assert!(matches!(packet.kind, PacketKind::LessThan(..)));
    }

    #[test]
    fn test_length_type_one() {
        let input = "EE00D40C823060";
        let packet = Packet::from_input(input);
        assert_eq!(packet.version, 7);
        assert!(matches!(packet.kind, PacketKind::Maximum(..)));
        if let PacketKind::Maximum(v) = &packet.kind {
            assert_eq!(v.len(), 3);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("8A004A801A8002F478"), 16);
        assert_eq!(part1("620080001611562C8802118E34"), 12);
        assert_eq!(part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("C200B40A82"), 3);
        assert_eq!(part2("04005AC33890"), 54);
        assert_eq!(part2("880086C3E88112"), 7);
        assert_eq!(part2("CE00C43D881120"), 9);
        assert_eq!(part2("D8005AC2A8F0"), 1);
        assert_eq!(part2("F600BC2D8F"), 0);
        assert_eq!(part2("9C005AC2F8F0"), 0);
        assert_eq!(part2("9C0141080250320F1802104A08"), 1);
    }
}

fn main() -> Result<()> {
    let input = include_str!("day16_input.txt").trim_end();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
    Ok(())
}
