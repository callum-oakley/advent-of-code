use std::{collections::HashMap, iter};

use crate::grid::{Adjacent, E, N, S, Vector, W, Z};

fn spiral() -> impl Iterator<Item = Vector> {
    let mut v = W;
    let mut ring = 0;
    let mut dir = E;
    let mut steps = 2;
    iter::from_fn(move || {
        v += dir;
        steps -= 1;
        if steps == 0 {
            if dir == N {
                dir = W;
                steps = 2 * ring;
            } else if dir == W {
                dir = S;
                steps = 2 * ring;
            } else if dir == S {
                dir = E;
                steps = 2 * ring + 1;
            } else if dir == E {
                ring += 1;
                dir = N;
                steps = 2 * ring - 1;
            }
        }
        Some(v)
    })
}

pub fn part1(input: &str) -> i64 {
    spiral()
        .nth(input.parse::<usize>().unwrap() - 1)
        .unwrap()
        .abs()
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let input = input.parse().unwrap();

    let mut mem = HashMap::new();
    mem.insert(Z, 1);

    for square in spiral().skip(1) {
        let value = square.adjacent8().filter_map(|v| mem.get(&v)).sum();
        if value > input {
            return value;
        }
        mem.insert(square, value);
    }

    unreachable!();
}

pub fn tests() {
    assert_eq!(part1("1"), 0);
    assert_eq!(part1("12"), 3);
    assert_eq!(part1("23"), 2);
    assert_eq!(part1("1024"), 31);
}
