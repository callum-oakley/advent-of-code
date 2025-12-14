use std::collections::{HashMap, HashSet};

use crate::grid::{Vector, Z};

fn parse_wire(s: &str) -> impl Iterator<Item = Vector> + '_ {
    s.split(',')
        .flat_map(|instruction| {
            let (dir, n) = instruction.split_at(1);
            std::iter::repeat_n(
                crate::cast::char_to_vector(dir.chars().next().unwrap()),
                n.parse().unwrap(),
            )
        })
        .scan(Z, |pos, dir| {
            *pos += dir;
            Some(*pos)
        })
}

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = Vector> + '_,
    impl Iterator<Item = Vector> + '_,
) {
    let (wire0, wire1) = input.split_once('\n').unwrap();
    (parse_wire(wire0), parse_wire(wire1))
}

pub fn part1(input: &str) -> i64 {
    let (wire0, wire1) = parse(input);
    let wire0 = wire0.collect::<HashSet<Vector>>();
    wire1
        .filter(|pos| wire0.contains(pos))
        .map(|v| v.abs().sum())
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let (wire0, wire1) = parse(input);
    let wire0 = wire0
        .enumerate()
        .map(|(steps0, pos)| (pos, steps0))
        .collect::<HashMap<Vector, usize>>();
    wire1
        .enumerate()
        .filter_map(|(steps1, pos)| {
            wire0.get(&pos).map(|steps0| {
                // We don't include Z in the wires so add the initial step for each
                steps0 + steps1 + 2
            })
        })
        .min()
        .unwrap()
}

pub fn tests() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    assert_eq!(
        part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        159,
    );
    assert_eq!(
        part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        135,
    );

    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    assert_eq!(
        part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        610,
    );
    assert_eq!(
        part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        410,
    );
}
