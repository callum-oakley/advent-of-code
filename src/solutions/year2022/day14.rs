use std::collections::{HashSet, VecDeque};

use crate::{
    grid::{Bounds, Vector, N, NE, NW, S, SE, SW},
    part::Part,
};

fn parse(input: &str) -> HashSet<Vector> {
    let mut rock = HashSet::new();
    for line in input.trim().lines() {
        let points: Vec<Vector> = line
            .trim()
            .split(" -> ")
            .map(crate::cast::string_to_vector)
            .collect();
        for pair in points.windows(2) {
            rock.extend(crate::grid::line_segment(pair[0], pair[1]));
        }
    }
    rock
}

// Adapted from 2018 day 17
fn part_(part: Part, input: &str) -> usize {
    let rock = parse(input);
    let bounds = Bounds::from(&rock);

    let mut flowing = HashSet::from([Vector::new(500, 0)]);
    let mut settled = HashSet::new();
    let mut queue = VecDeque::from([Vector::new(500, 0)]);

    while let Some(block) = queue.pop_front() {
        if flowing.contains(&block) && block.y <= bounds.max.y + 1 {
            if let Some(v) = [block + S, block + SW, block + SE].into_iter().find(|&v| {
                !(rock.contains(&v)
                    || settled.contains(&v)
                    || part == Part::Two && v.y == bounds.max.y + 2)
            }) {
                if !flowing.contains(&v) {
                    flowing.insert(v);
                    queue.push_back(v);
                }
            } else {
                flowing.remove(&block);
                settled.insert(block);
                queue.extend([block + N, block + NW, block + NE]);
            }
        }
    }

    settled.len()
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    ";
    assert_eq!(part1(example), 24);
    assert_eq!(part2(example), 93);
}
