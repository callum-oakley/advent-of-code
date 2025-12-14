use std::collections::HashSet;

use crate::grid::{Vector, Z};

struct Motion {
    dir: Vector,
    dist: usize,
}

fn parse(input: &str) -> impl Iterator<Item = Motion> {
    input.lines().map(|line| {
        let (dir, dist) = line.split_once(' ').unwrap();
        Motion {
            dir: crate::cast::str_to_vector(dir),
            dist: dist.parse().unwrap(),
        }
    })
}

fn follow(head: Vector, tail: Vector) -> Vector {
    if (head - tail).abs().max() > 1 {
        tail + (head - tail).map(i64::signum)
    } else {
        tail
    }
}

fn simulate(input: &str, rope_len: usize) -> usize {
    let mut rope = vec![Z; rope_len];
    let mut visited = HashSet::from([Z]);

    for motion in parse(input) {
        for _ in 0..motion.dist {
            rope[0] += motion.dir;
            for i in 1..rope.len() {
                rope[i] = follow(rope[i - 1], rope[i]);
            }
            visited.insert(rope[rope.len() - 1]);
        }
    }

    visited.len()
}

pub fn part1(input: &str) -> usize {
    simulate(input, 2)
}

pub fn part2(input: &str) -> usize {
    simulate(input, 10)
}

pub fn tests() {
    assert_eq!(part1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 13);
    assert_eq!(part2("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 1);
    assert_eq!(part2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"), 36);
}
