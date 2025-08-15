use crate::grid::{E, N, S, Z};

fn parse(input: &str) -> impl Iterator<Item = (&str, i64)> + '_ {
    input.trim().lines().map(|line| {
        let (dir, n) = line.trim().split_once(' ').unwrap();
        (dir, n.parse().unwrap())
    })
}

pub fn part1(input: &str) -> i64 {
    let mut pos = Z;
    for (dir, n) in parse(input) {
        match dir {
            "forward" => pos += n * E,
            "down" => pos += n * S,
            "up" => pos += n * N,
            _ => unreachable!(),
        }
    }
    pos.x * pos.y
}

pub fn part2(input: &str) -> i64 {
    let mut pos = Z;
    let mut aim = E;
    for (dir, n) in parse(input) {
        match dir {
            "forward" => pos += n * aim,
            "down" => aim += n * S,
            "up" => aim += n * N,
            _ => unreachable!(),
        }
    }
    pos.x * pos.y
}

pub fn tests() {
    let example = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";
    assert_eq!(part1(example), 150);
    assert_eq!(part2(example), 900);
}
