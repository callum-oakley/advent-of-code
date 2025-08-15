use std::collections::HashSet;

use crate::{
    grid::{Adjacent, IntoVector, Vector, Z},
    search::{self, hash_filter},
};

fn parse(input: &str) -> Vec<Vector> {
    input
        .split_whitespace()
        .map(IntoVector::into_vector)
        .collect()
}

fn search(size: i64, corrupted: &HashSet<Vector>) -> Option<usize> {
    search::breadth_first(
        (Z, 0),
        |&(pos, steps), push| {
            pos.adjacent4()
                .filter(|a| {
                    a.x >= 0 && a.y >= 0 && a.x <= size && a.y <= size && !corrupted.contains(a)
                })
                .for_each(|a| push((a, steps + 1)));
        },
        hash_filter(|&(pos, _)| pos),
    )
    .find(|&(pos, _)| pos == Vector::new(size, size))
    .map(|(_, steps)| steps)
}

fn part1_(size: i64, bytes: usize, input: &str) -> usize {
    search(size, &parse(input)[..bytes].iter().copied().collect()).unwrap()
}

pub fn part1(input: &str) -> usize {
    part1_(70, 1024, input)
}

fn part2_(size: i64, input: &str) -> String {
    let bytes = parse(input);
    let i = search::binary(0, bytes.len(), |i| {
        search(size, &bytes[..i].iter().copied().collect()).is_none()
    }) - 1;
    format!("{},{}", bytes[i].x, bytes[i].y)
}

pub fn part2(input: &str) -> String {
    part2_(70, input)
}

pub fn tests() {
    let example = "
        5,4 4,2 4,5 3,0 2,1 6,3 2,4 1,5 0,6 3,3 2,6 5,1 1,2
        5,5 2,5 6,5 1,4 0,4 6,4 1,1 6,1 1,0 0,5 1,6 2,0
    ";
    assert_eq!(part1_(6, 12, example), 22);
    assert_eq!(part2_(6, example), "6,1");
}
