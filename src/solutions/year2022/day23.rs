use std::collections::{HashMap, HashSet};

use crate::grid::{Adjacent, Bounds, E, N, NE, NW, S, SE, SW, Vector, W};

fn step(round: usize, elves: &HashSet<Vector>) -> HashSet<Vector> {
    let dirs = [(NW, N, NE), (SE, S, SW), (SW, W, NW), (NE, E, SE)];

    let mut elves = elves.clone();
    let mut proposed: HashMap<Vector, Vec<Vector>> = HashMap::new();
    for &elf in &elves {
        if elf.adjacent8().any(|a| elves.contains(&a)) {
            for (left, dir, right) in (0..4).map(|i| dirs[(i + round) % dirs.len()]) {
                if ![left, dir, right]
                    .iter()
                    .any(|v| elves.contains(&(elf + v)))
                {
                    proposed.entry(elf + dir).or_default().push(elf);
                    break;
                }
            }
        }
    }

    for (to, from) in proposed {
        if from.len() == 1 {
            elves.remove(&from[0]);
            elves.insert(to);
        }
    }

    elves
}

pub fn part1(input: &str) -> usize {
    let mut elves = crate::cast::str_to_vector_hash_set(input);
    for round in 0..10 {
        elves = step(round, &elves);
    }
    let bounds = Bounds::from(&elves);
    usize::try_from(bounds.size().x * bounds.size().y).unwrap() - elves.len()
}

pub fn part2(input: &str) -> usize {
    let mut elves = crate::cast::str_to_vector_hash_set(input);
    let mut round = 0;
    loop {
        let elves_next = step(round, &elves);
        round += 1;
        if elves_next == elves {
            return round;
        }
        elves = elves_next;
    }
}

pub fn tests() {
    let example = "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..";
    assert_eq!(part1(example), 110);
    assert_eq!(part2(example), 20);
}
