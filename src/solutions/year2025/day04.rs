use std::collections::HashSet;

use crate::grid::{Adjacent, Vector};

fn parse(input: &str) -> HashSet<Vector> {
    crate::grid::scan(input)
        .filter(|&(_, c)| c == '@')
        .map(|(v, _)| v)
        .collect()
}

fn accessible(rolls: &HashSet<Vector>) -> impl Iterator<Item = Vector> {
    rolls
        .iter()
        .filter(|&r| r.adjacent8().filter(|a| rolls.contains(a)).count() < 4)
        .copied()
}

pub fn part1(input: &str) -> usize {
    accessible(&parse(input)).count()
}

pub fn part2(input: &str) -> usize {
    let mut rolls = parse(input);

    let mut removed = 0;
    while let accessible = accessible(&rolls).collect::<Vec<_>>()
        && !accessible.is_empty()
    {
        for a in &accessible {
            rolls.remove(a);
        }
        removed += accessible.len();
    }

    removed
}

pub fn tests() {
    let example = concat!(
        "..@@.@@@@.\n",
        "@@@.@.@.@@\n",
        "@@@@@.@.@@\n",
        "@.@@@@..@.\n",
        "@@.@@@@.@@\n",
        ".@@@@@@@.@\n",
        ".@.@.@.@@@\n",
        "@.@@@.@@@@\n",
        ".@@@@@@@@.\n",
        "@.@.@@@.@.\n",
    );
    assert_eq!(part1(example), 13);
    assert_eq!(part2(example), 43);
}
