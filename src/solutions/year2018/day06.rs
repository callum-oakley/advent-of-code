use std::collections::HashMap;

use crate::grid::{Bounds, IntoVector, Vector};

fn unique_closest(pos: Vector, coordinates: &[Vector]) -> Option<Vector> {
    let mut min_dist = (pos - coordinates[0]).abs().sum();
    let mut res = Some(coordinates[0]);
    for &c in &coordinates[1..] {
        let dist = (pos - c).abs().sum();
        match dist.cmp(&min_dist) {
            std::cmp::Ordering::Less => {
                min_dist = dist;
                res = Some(c);
            }
            std::cmp::Ordering::Equal => {
                res = None;
            }
            std::cmp::Ordering::Greater => {}
        }
    }
    res
}

pub fn part1(input: &str) -> u32 {
    let coordinates = input
        .lines()
        .map(IntoVector::into_vector)
        .collect::<Vec<_>>();

    let mut areas = coordinates
        .iter()
        .map(|c| (c, 0))
        .collect::<HashMap<_, _>>();

    let bounds = Bounds::<2>::new(coordinates.iter().copied());
    for x in bounds.min.x..=bounds.max.x {
        for y in bounds.min.y..=bounds.max.y {
            if let Some(c) = unique_closest(Vector::new(x, y), &coordinates) {
                if x == bounds.min.x || x == bounds.max.x || y == bounds.min.y || y == bounds.max.y
                {
                    // This area is inifinite, so we don't care about it.
                    areas.remove(&c);
                } else if let Some(v) = areas.get_mut(&c) {
                    *v += 1;
                }
            }
        }
    }

    areas.into_values().max().unwrap()
}

fn part2_(tolerance: i64, input: &str) -> u32 {
    let coordinates = input
        .lines()
        .map(IntoVector::into_vector)
        .collect::<Vec<_>>();

    let mut res = 0;

    let bounds = Bounds::<2>::new(coordinates.iter().copied());
    for x in bounds.min.x..=bounds.max.x {
        for y in bounds.min.y..=bounds.max.y {
            let pos = Vector::new(x, y);
            if coordinates
                .iter()
                .map(|&c| (pos - c).abs().sum())
                .sum::<i64>()
                < tolerance
            {
                res += 1;
            }
        }
    }

    res
}

pub fn part2(input: &str) -> u32 {
    part2_(10_000, input)
}

pub fn tests() {
    let example = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
    assert_eq!(part1(example), 17);
    assert_eq!(part2_(32, example), 16);
}
