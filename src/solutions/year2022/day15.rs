use regex::Regex;

use crate::{grid::Vector, uniq::Uniq};

#[derive(Clone, Copy)]
struct Reading {
    sensor: Vector,
    beacon: Vector,
}

fn parse(input: &str) -> Vec<Reading> {
    Regex::new(r"Sensor at (x=-?\d+, y=-?\d+): closest beacon is at (x=-?\d+, y=-?\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| Reading {
            sensor: crate::cast::string_to_vector(&captures[1]),
            beacon: crate::cast::string_to_vector(&captures[2]),
        })
        .collect()
}

// Cheating a little by assuming there are no gaps. This turns out to be true.
fn part1_(y: i64, input: &str) -> usize {
    let readings = parse(input);
    let min_x = readings
        .iter()
        .map(|Reading { sensor, beacon }| {
            let r = (sensor - beacon).abs().sum();
            sensor.x - (r - (sensor.y - y).abs())
        })
        .min()
        .unwrap();
    let max_x = readings
        .iter()
        .map(|Reading { sensor, beacon }| {
            let r = (sensor - beacon).abs().sum();
            sensor.x + (r - (sensor.y - y).abs())
        })
        .max()
        .unwrap();
    let beacons = readings
        .iter()
        .map(|reading| reading.beacon)
        .filter(|beacon| beacon.y == y)
        .uniq()
        .count();
    usize::try_from(max_x + 1 - min_x).unwrap() - beacons
}

fn boundary_points(a: Reading, b: Reading) -> impl Iterator<Item = Vector> {
    let r_a = (a.sensor - a.beacon).abs().sum() + 1;
    let r_b = (b.sensor - b.beacon).abs().sum() + 1;
    [
        (a.sensor, r_a, b.sensor, r_b),
        (b.sensor, r_b, a.sensor, r_a),
    ]
    .into_iter()
    .flat_map(|(s0, r0, s1, r1)| {
        [1, -1]
            .into_iter()
            .map(move |sign0| (s0, r0, s1, r1, sign0))
    })
    .flat_map(|(s0, r0, s1, r1, sign0)| {
        [1, -1]
            .into_iter()
            .map(move |sign1| (s0, r0, s1, r1, sign0, sign1))
    })
    .filter_map(|(s0, r0, s1, r1, sign0, sign1)| {
        let a = s0.y - s0.x - sign0 * r0;
        let b = s1.y + s1.x + sign1 * r1;
        let v = Vector::new((b - a) / 2, i64::midpoint(b, a));
        if (b + a) % 2 == 0 && (v - s0).abs().sum() == r0 && (v - s1).abs().sum() == r1 {
            Some(v)
        } else {
            None
        }
    })
}

// Since we know there is a unique solution it must be on the boundary of two sensors (one unit
// further away than the detected beacon) otherwise nearby points would also be valid solutions.
fn part2_(bound: i64, input: &str) -> i64 {
    let readings = parse(input);
    let v = crate::combinatorics::combinations(2, &readings)
        .flat_map(|pair| boundary_points(*pair[0], *pair[1]))
        .find(|v| {
            v.iter().all(|&a| 0 <= a && a <= bound)
                && readings.iter().all(|reading| {
                    (reading.sensor - reading.beacon).abs().sum() < (reading.sensor - v).abs().sum()
                })
        })
        .unwrap();
    v.x * 4_000_000 + v.y
}

pub fn part1(input: &str) -> usize {
    part1_(2_000_000, input)
}

pub fn part2(input: &str) -> i64 {
    part2_(4_000_000, input)
}

pub fn tests() {
    let example = "
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";
    assert_eq!(part1_(10, example), 26);
    assert_eq!(part2_(20, example), 56_000_011);
}
