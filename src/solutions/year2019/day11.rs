use std::collections::{HashMap, HashSet};

use crate::{
    grid::{LEFT, N, RIGHT, Vector, Z},
    intcode::{State, VM},
};

fn paint(input: &str, hull: &mut HashMap<Vector, i64>) {
    let mut pos = Z;
    let mut dir = N;
    let mut vm = VM::new(input);
    while vm.state() != State::Halt {
        vm.input(hull.get(&pos).copied().unwrap_or_default());
        hull.insert(pos, vm.output());
        let turn = match vm.output() {
            0 => LEFT,
            1 => RIGHT,
            _ => unreachable!(),
        };
        dir = turn * dir;
        pos += dir;
    }
}

pub fn part1(input: &str) -> usize {
    let mut hull = HashMap::new();
    paint(input, &mut hull);
    hull.len()
}

pub fn part2(input: &str) -> &str {
    let mut hull = HashMap::from([(Z, 1)]);
    paint(input, &mut hull);
    crate::ocr::parse(&crate::cast::vector_hash_set_to_string(
        &hull
            .iter()
            .filter(|&(_, &paint)| paint == 1)
            .map(|(&pos, _)| pos)
            .collect::<HashSet<_>>(),
    ))
}
