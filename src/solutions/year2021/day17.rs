use regex::Regex;

use crate::grid::{Bounds, Vector, Z};

fn parse(input: &str) -> Bounds<2> {
    let captures = Regex::new(r"target area: x=(\d+)..(\d+), y=-(\d+)..-(\d+)")
        .unwrap()
        .captures(input)
        .unwrap();
    let min_x = captures[1].parse().unwrap();
    let max_x = captures[2].parse().unwrap();
    let min_y = captures[4].parse().unwrap();
    let max_y = captures[3].parse().unwrap();
    Bounds {
        min: Vector::new(min_x, min_y),
        max: Vector::new(max_x, max_y),
    }
}

#[derive(Clone, Copy)]
struct Probe {
    pos: Vector,
    vel: Vector,
}

fn step(mut probe: Probe) -> Probe {
    probe.pos += probe.vel;
    if probe.vel.x > 0 {
        probe.vel.x -= 1;
    }
    probe.vel.y += 1;
    probe
}

fn hits(target: Bounds<2>, mut probe: Probe) -> bool {
    while probe.pos.x <= target.max.x && probe.pos.y <= target.max.y {
        if target.contains(probe.pos) {
            return true;
        }
        probe = step(probe);
    }
    false
}

fn max_height(probe: Probe) -> i64 {
    probe.vel.y * (probe.vel.y - 1) / 2
}

fn part_(input: &str) -> impl Iterator<Item = Probe> {
    let target = parse(input);
    (0..=target.max.x)
        .flat_map(move |x| {
            (-target.max.y..=target.max.y).map(move |y| Probe {
                pos: Z,
                vel: Vector::new(x, y),
            })
        })
        .filter(move |&probe| hits(target, probe))
}

pub fn part1(input: &str) -> i64 {
    part_(input).map(max_height).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    part_(input).count()
}

pub fn tests() {
    let example = "target area: x=20..30, y=-10..-5";
    assert_eq!(part1(example), 45);
    assert_eq!(part2(example), 112);
}
