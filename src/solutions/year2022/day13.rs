use std::cmp::Ordering;

use crate::sexp::{Inner, Value};

fn parse(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a.as_inner(), b.as_inner()) {
        (Inner::Int(a), Inner::Int(b)) => a.cmp(b),
        (Inner::Int(_), Inner::Vec(_)) => compare(&Value::vec(vec![a.clone()]), b),
        (Inner::Vec(_), Inner::Int(_)) => compare(a, &Value::vec(vec![b.clone()])),
        (Inner::Vec(a), Inner::Vec(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();
            loop {
                match (a.next(), b.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(a), Some(b)) => match compare(a, b) {
                        Ordering::Equal => {}
                        o => return o,
                    },
                }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| compare(&pair[0], &pair[1]) != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = parse(input);
    let dividers = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    packets.extend(dividers.clone());
    packets.sort_unstable_by(compare);
    (packets.iter().position(|p| p == &dividers[0]).unwrap() + 1)
        * (packets.iter().position(|p| p == &dividers[1]).unwrap() + 1)
}

pub fn tests() {
    let example = "
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";
    assert_eq!(part1(example), 13);
    assert_eq!(part2(example), 140);
}
