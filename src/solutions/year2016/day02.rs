use std::collections::HashMap;

use crate::grid::{E, N, NE, NW, S, SE, SW, Vector, W, Z};

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = Vector> + '_> + '_ {
    input
        .split_whitespace()
        .map(|s| s.chars().map(crate::cast::char_to_vector))
}

fn part_(keypad: &HashMap<Vector, char>, input: &str) -> String {
    let mut res = String::new();
    let mut pos = Z;
    for instruction in parse(input) {
        for dir in instruction {
            if keypad.contains_key(&(pos + dir)) {
                pos += dir;
            }
        }
        res.push(keypad[&pos]);
    }
    res
}

pub fn part1(input: &str) -> String {
    part_(
        &[
            (NW, '1'),
            (N, '2'),
            (NE, '3'),
            (W, '4'),
            (Z, '5'),
            (E, '6'),
            (SW, '7'),
            (S, '8'),
            (SE, '9'),
        ]
        .into(),
        input,
    )
}

pub fn part2(input: &str) -> String {
    part_(
        &[
            (Vector::new(2, -2), '1'),
            (Vector::new(1, -1), '2'),
            (Vector::new(2, -1), '3'),
            (Vector::new(3, -1), '4'),
            (Vector::new(0, 0), '5'),
            (Vector::new(1, 0), '6'),
            (Vector::new(2, 0), '7'),
            (Vector::new(3, 0), '8'),
            (Vector::new(4, 0), '9'),
            (Vector::new(1, 1), 'A'),
            (Vector::new(2, 1), 'B'),
            (Vector::new(3, 1), 'C'),
            (Vector::new(2, 2), 'D'),
        ]
        .into(),
        input,
    )
}

pub fn tests() {
    assert_eq!(part1("ULL RRDDD LURDL UUUUD"), "1985");
    assert_eq!(part2("ULL RRDDD LURDL UUUUD"), "5DB3");
}
