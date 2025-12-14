use std::collections::HashSet;

use crate::{
    grid::{Grid, N, RIGHT, Vector},
    uniq::Uniq,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    pos: Vector,
    dir: Vector,
}

fn parse(input: &str) -> (Grid<bool>, Guard) {
    let mut guard = None;
    let obstructions = Grid::parse(input, |pos, c| match c {
        '#' => true,
        '.' => false,
        '^' => {
            guard = Some(Guard { pos, dir: N });
            false
        }
        _ => unreachable!(),
    });
    (obstructions, guard.unwrap())
}

fn walk(obstructions: &Grid<bool>, mut guard: Guard) -> (HashSet<Guard>, bool) {
    let mut visited = HashSet::new();
    while obstructions.contains_key(guard.pos) {
        if visited.contains(&guard) {
            return (visited, true);
        }

        visited.insert(guard);
        if obstructions.get(guard.pos + guard.dir) == Some(&true) {
            guard.dir = RIGHT * guard.dir;
        } else {
            guard.pos += guard.dir;
        }
    }

    (visited, false)
}

pub fn part1(input: &str) -> usize {
    let (obstructions, guard) = parse(input);
    walk(&obstructions, guard)
        .0
        .into_iter()
        .map(|guard| guard.pos)
        .uniq()
        .count()
}

pub fn part2(input: &str) -> usize {
    let (mut obstructions, guard) = parse(input);

    let path = walk(&obstructions, guard)
        .0
        .into_iter()
        .map(|guard| guard.pos)
        .collect::<HashSet<_>>();

    let mut res = 0;
    for pos in path.into_iter().filter(|&pos| pos != guard.pos) {
        obstructions[pos] = true;
        if walk(&obstructions, guard).1 {
            res += 1;
        }
        obstructions[pos] = false;
    }

    res
}

pub fn tests() {
    let example = [
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ]
    .join("\n");
    assert_eq!(part1(&example), 41);
    assert_eq!(part2(&example), 6);
}
