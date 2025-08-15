use std::collections::HashSet;

use nalgebra::SVector;

use crate::{grid::Grid, uniq::Uniq};

fn neighbors<const D: usize>(v: SVector<i64, D>) -> Vec<SVector<i64, D>> {
    let mut res = vec![v];
    for axis in 0..v.len() {
        let dir = SVector::from_fn(|i, _| (i == axis).into());
        res = res
            .into_iter()
            .flat_map(|v| [v - dir, v, v + dir])
            .collect();
    }
    res.retain(|&u| u != v);
    res
}

fn part_<const D: usize>(mut active: HashSet<SVector<i64, D>>) -> usize {
    for _ in 0..6 {
        active = active
            .iter()
            .copied()
            .flat_map(neighbors)
            .uniq()
            .filter(|&v| {
                let active_neighbors = neighbors(v)
                    .into_iter()
                    .filter(|n| active.contains(n))
                    .count();
                if active.contains(&v) {
                    (2..=3).contains(&active_neighbors)
                } else {
                    active_neighbors == 3
                }
            })
            .collect();
    }
    active.len()
}

pub fn part1(input: &str) -> usize {
    part_(
        Grid::parse(input, |_, c| c == '#')
            .points()
            .map(|v| v.insert_row(2, 0))
            .collect(),
    )
}

pub fn part2(input: &str) -> usize {
    part_(
        Grid::parse(input, |_, c| c == '#')
            .points()
            .map(|v| v.insert_row(2, 0).insert_row(3, 0))
            .collect(),
    )
}

pub fn tests() {
    assert_eq!(part1(".#.\n..#\n###"), 112);
    assert_eq!(part2(".#.\n..#\n###"), 848);
}
