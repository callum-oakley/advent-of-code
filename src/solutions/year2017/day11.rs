use std::cmp;

use crate::grid::{Vector, Z};

// As described here (in the "flat" orientation):
// <https://www.redblobgames.com/grids/hexagons/#coordinates-axial>
// Let x correspond to q and y correspond to r:
fn into_hex_vector(s: &str) -> Vector {
    match s {
        "n" => Vector::new(0, -1),
        "ne" => Vector::new(1, -1),
        "se" => Vector::new(1, 0),
        "s" => Vector::new(0, 1),
        "sw" => Vector::new(-1, 1),
        "nw" => Vector::new(-1, 0),
        _ => unreachable!(),
    }
}

fn hex_dist(v: Vector) -> i64 {
    (v.x.abs() + v.y.abs() + (v.x + v.y).abs()) / 2
}

pub fn part1(input: &str) -> i64 {
    hex_dist(input.split(',').map(into_hex_vector).sum::<Vector>())
}

pub fn part2(input: &str) -> i64 {
    let mut furthest = 0;
    let mut v = Z;
    for step in input.split(',').map(into_hex_vector) {
        v += step;
        furthest = cmp::max(furthest, hex_dist(v));
    }
    furthest
}

pub fn tests() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}
