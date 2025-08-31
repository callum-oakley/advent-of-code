use std::collections::HashSet;

use crate::grid::{Turn, N, Z};

fn parse(input: &str) -> impl Iterator<Item = (Turn, i64)> + '_ {
    input.split(", ").map(|instruction| {
        (
            crate::cast::string_to_turn(&instruction[0..1]),
            instruction[1..].parse().unwrap(),
        )
    })
}

pub fn part1(input: &str) -> i64 {
    parse(input)
        .fold((Z, N), |(pos, dir), (turn, dist)| {
            let dir = turn * dir;
            (pos + dir * dist, dir)
        })
        .0
        .abs()
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut pos = Z;
    let mut dir = N;
    let mut visited = HashSet::from([Z]);
    for (turn, dist) in parse(input) {
        dir = turn * dir;
        for _ in 0..dist {
            pos += dir;
            if visited.contains(&pos) {
                return pos.abs().sum();
            }
            visited.insert(pos);
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(part1("R2, L3"), 5);
    assert_eq!(part1("R2, R2, R2"), 2);
    assert_eq!(part1("R5, L5, R5, R3"), 12);

    assert_eq!(part2("R8, R4, R4, R8"), 4);
}
