use std::cmp::Ordering;

use crate::sexp::Value;

fn parse(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a.is_int(), b.is_int()) {
        (true, true) => a.unint().cmp(&b.unint()),
        (true, false) => compare(&Value::from([a]), b),
        (false, true) => compare(a, &Value::from([b])),
        (false, false) => match (a.is_nil(), b.is_nil()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => match compare(a.head(), b.head()) {
                Ordering::Equal => compare(a.tail(), b.tail()),
                o => o,
            },
        },
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
