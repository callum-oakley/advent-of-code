use std::{cmp::Ordering, iter::Peekable, str::Chars};

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::Int(a), _) => Packet::List(vec![Packet::Int(*a)]).cmp(other),
            (_, Packet::Int(b)) => self.cmp(&Packet::List(vec![Packet::Int(*b)])),
            (Packet::List(a), Packet::List(b)) => {
                let mut a_it = a.iter();
                let mut b_it = b.iter();
                loop {
                    match (a_it.next(), b_it.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            o => return o,
                        },
                    }
                }
            }
        }
    }
}

fn parse(input: &str) -> Vec<Packet> {
    parse_packets(&mut input.chars().peekable())
}

fn parse_packets(chars: &mut Peekable<Chars>) -> Vec<Packet> {
    let mut packets = Vec::new();
    while let Some(packet) = parse_packet(chars) {
        packets.push(packet);
    }
    packets
}

fn parse_packet(chars: &mut Peekable<Chars>) -> Option<Packet> {
    while chars.peek().is_some_and(|&c| c.is_whitespace() || c == ',') {
        chars.next();
    }
    match chars.peek() {
        Some('[') => {
            chars.next();
            let packet = Packet::List(parse_packets(chars));
            assert_eq!(chars.next(), Some(']'));
            Some(packet)
        }
        Some('0'..='9') => {
            let mut n = String::new();
            while chars.peek().is_some_and(char::is_ascii_digit) {
                n.push(chars.next().unwrap());
            }
            Some(Packet::Int(n.parse().unwrap()))
        }
        _ => None,
    }
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] <= pair[1])
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = parse(input);
    let dividers = parse("[[2]] [[6]]");
    packets.extend(dividers.clone());
    packets.sort_unstable();
    (packets.iter().position(|p| p == &dividers[0]).unwrap() + 1)
        * (packets.iter().position(|p| p == &dividers[1]).unwrap() + 1)
}

pub fn tests() {
    let example = "
        [1,1,3,1,1] [1,1,5,1,1]
        [[1],[2,3,4]] [[1],4]
        [9] [[8,7,6]]
        [[4,4],4,4] [[4,4],4,4,4]
        [7,7,7,7] [7,7,7]
        [] [3]
        [[[]]] [[]]
        [1,[2,[3,[4,[5,6,7]]]],8,9] [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";
    assert_eq!(part1(example), 13);
    assert_eq!(part2(example), 140);
}
