use std::collections::HashSet;

use nalgebra::{vector, Vector3};

use crate::grid::{Adjacent3, Bounds, IntoVector};

fn parse(input: &str) -> HashSet<Vector3<i64>> {
    input.trim().lines().map(IntoVector::into_vector).collect()
}

pub fn part1(input: &str) -> usize {
    let cubes = parse(input);
    cubes
        .iter()
        .flat_map(|&cube| cube.adjacent6())
        .filter(|side| !cubes.contains(side))
        .count()
}

pub fn part2(input: &str) -> usize {
    let cubes = parse(input);
    let mut bounds = Bounds::from(&cubes);
    bounds.min -= vector![1, 1, 1];
    bounds.max += vector![1, 1, 1];
    let exterior: HashSet<_> = crate::search::breadth_first(
        bounds.min,
        |cube, push| {
            cube.adjacent6()
                .filter(|&c| bounds.contains(c) && !cubes.contains(&c))
                .for_each(push);
        },
        crate::search::id_filter(),
    )
    .collect();
    cubes
        .iter()
        .flat_map(|&cube| cube.adjacent6())
        .filter(|side| exterior.contains(side))
        .count()
}

pub fn tests() {
    let example = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    ";
    assert_eq!(part1(example), 64);
    assert_eq!(part2(example), 58);
}
