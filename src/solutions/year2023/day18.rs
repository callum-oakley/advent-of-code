use lazy_static::lazy_static;
use regex::Regex;

use crate::grid::{Point, E, N, S, W, Z};

lazy_static! {
    static ref RE: Regex = Regex::new(r"([UDLR]) (\d+) \(#([0-9a-f]{5})([0-3])\)").unwrap();
}

fn parse1(input: &str) -> impl Iterator<Item = (Point, i32)> + '_ {
    RE.captures_iter(input)
        .map(|captures| (captures[1].into(), captures[2].parse().unwrap()))
}

fn parse2(input: &str) -> impl Iterator<Item = (Point, i32)> + '_ {
    RE.captures_iter(input).map(|captures| {
        (
            match &captures[4] {
                "0" => E,
                "1" => S,
                "2" => W,
                "3" => N,
                _ => unreachable!(),
            },
            i32::from_str_radix(&captures[3], 16).unwrap(),
        )
    })
}

fn part_(input: impl Iterator<Item = (Point, i32)>) -> i64 {
    let mut pos = Z;
    let mut boundary = vec![pos];
    let mut boundary_len = 0;
    for (dir, side_len) in input {
        pos += dir * side_len;
        boundary.push(pos);
        boundary_len += i64::from(side_len);
    }

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;
    for i in 0..boundary.len() - 1 {
        area += i64::from(boundary[i].y + boundary[i + 1].y)
            * i64::from(boundary[i].x - boundary[i + 1].x);
    }
    area /= 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    area + boundary_len / 2 + 1
}

pub fn part1(input: &str) -> i64 {
    part_(parse1(input))
}

pub fn part2(input: &str) -> i64 {
    part_(parse2(input))
}

pub fn tests() {
    let example = [
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ]
    .join("\n");
    assert_eq!(part1(&example), 62);
    assert_eq!(part2(&example), 952_408_144_115);
}
