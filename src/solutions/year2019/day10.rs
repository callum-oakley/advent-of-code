use std::collections::HashMap;

use ordered_float::OrderedFloat;

use crate::{grid::Vector, uniq::Uniq};

fn parse(input: &str) -> Vec<Vector> {
    let mut res = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                res.push(Vector::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    res
}

// Angle from the y-axis, in the range (-pi, pi] with zero straight down, and increasing clockwise.
// Note that this is shifted from the usual definition used in polar coordinates because it makes
// the problem easier.
fn theta(asteroid: Vector) -> OrderedFloat<f64> {
    OrderedFloat(-f64::atan2(
        i32::try_from(asteroid.x).unwrap().into(),
        i32::try_from(asteroid.y).unwrap().into(),
    ))
}

fn count_visible(asteroids: &[Vector], origin: Vector) -> usize {
    asteroids
        .iter()
        .filter(|&&a| a != origin)
        .map(|&a| theta(a - origin))
        .uniq()
        .count()
}

fn origin(asteroids: &[Vector]) -> Vector {
    asteroids
        .iter()
        .copied()
        .max_by_key(|&origin| count_visible(asteroids, origin))
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let asteroids = parse(input);
    count_visible(&asteroids, origin(&asteroids))
}

pub fn part2(input: &str) -> i64 {
    let mut asteroids = parse(input);
    let origin = origin(&asteroids);

    let mut rays: HashMap<OrderedFloat<f64>, Vec<Vector>> = HashMap::new();
    for &asteroid in &asteroids {
        rays.entry(theta(asteroid - origin))
            .or_default()
            .push(asteroid);
    }

    let mut depths: HashMap<Vector, usize> = HashMap::new();
    for ray in rays.values_mut() {
        ray.sort_unstable_by_key(|&asteroid| (asteroid - origin).abs().sum());
        for (depth, &asteroid) in ray.iter().enumerate() {
            depths.insert(asteroid, depth);
        }
    }

    asteroids.sort_unstable_by_key(|&asteroid| (depths[&asteroid], theta(asteroid - origin)));

    asteroids[199].x * 100 + asteroids[199].y
}

pub fn tests() {
    assert_eq!(part1(".#..#\n.....\n#####\n....#\n...##"), 8);
    assert_eq!(
        part1(
            &[
                "......#.#.",
                "#..#.#....",
                "..#######.",
                ".#.#.###..",
                ".#..#.....",
                "..#....#.#",
                "#..#....#.",
                ".##.#..###",
                "##...#..#.",
                ".#....####",
            ]
            .join("\n"),
        ),
        33,
    );
    assert_eq!(
        part1(
            &[
                "#.#...#.#.",
                ".###....#.",
                ".#....#...",
                "##.#.#.#.#",
                "....#.#.#.",
                ".##..###.#",
                "..#...##..",
                "..##....##",
                "......#...",
                ".####.###.",
            ]
            .join("\n"),
        ),
        35,
    );
    assert_eq!(
        part1(
            &[
                ".#..#..###",
                "####.###.#",
                "....###.#.",
                "..###.##.#",
                "##.##.#.#.",
                "....###..#",
                "..#.#..#.#",
                "#..#.#.###",
                ".##...##.#",
                ".....#.#..",
            ]
            .join("\n"),
        ),
        41,
    );

    let big_example = &[
        ".#..##.###...#######",
        "##.############..##.",
        ".#.######.########.#",
        ".###.#######.####.#.",
        "#####.##.#.##.###.##",
        "..#####..#.#########",
        "####################",
        "#.####....###.#.#.##",
        "##.#################",
        "#####.##.###..####..",
        "..######..##.#######",
        "####.##.####...##..#",
        ".#####..#.######.###",
        "##...#.##########...",
        "#.##########.#######",
        ".####.#.###.###.#.##",
        "....##.##.###..#####",
        ".#.#.###########.###",
        "#.#.#.#####.####.###",
        "###.##.####.##.#..##",
    ]
    .join("\n");
    assert_eq!(part1(big_example), 210);
    assert_eq!(part2(big_example), 802);
}
