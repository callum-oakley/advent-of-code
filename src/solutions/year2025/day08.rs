use std::{cmp::Reverse, collections::HashSet};

use crate::grid::Vector3;

#[derive(PartialEq, Clone, Copy)]
enum Part {
    One(usize),
    Two,
}

fn parse(input: &str) -> Vec<Vector3> {
    input
        .trim()
        .lines()
        .map(crate::cast::str_to_vector3)
        .collect()
}

fn sq_dist(a: Vector3, b: Vector3) -> i64 {
    (a - b).map(|c| c * c).sum()
}

fn part_(part: Part, input: &str) -> usize {
    let nodes = parse(input);

    let mut pairs = Vec::new();
    for j in 0..nodes.len() {
        for i in 0..j {
            pairs.push((nodes[i], nodes[j]));
        }
    }
    pairs.sort_unstable_by_key(|&(a, b)| sq_dist(a, b));

    if let Part::One(limit) = part {
        pairs.truncate(limit);
    }

    let mut circuits: Vec<_> = nodes.iter().map(|&a| HashSet::from([a])).collect();
    for (a, b) in pairs {
        let i = (0..circuits.len())
            .find(|&i| circuits[i].contains(&a))
            .unwrap();
        let j = (0..circuits.len())
            .find(|&i| circuits[i].contains(&b))
            .unwrap();
        if i != j {
            let circuit = circuits.swap_remove(i.max(j));
            circuits[i.min(j)].extend(circuit);
        }

        if part == Part::Two && circuits.len() == 1 {
            return usize::try_from(a.x * b.x).unwrap();
        }
    }

    circuits.sort_unstable_by_key(|circuit| Reverse(circuit.len()));
    circuits.iter().take(3).map(HashSet::len).product()
}

pub fn part1(input: &str) -> usize {
    part_(Part::One(1000), input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    ";
    assert_eq!(part_(Part::One(10), example), 40);
    assert_eq!(part2(example), 25272);
}
