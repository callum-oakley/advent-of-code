use std::collections::HashSet;

use crate::grid::{Adjacent, Vector};

fn parse(input: &str) -> HashSet<Vector> {
    crate::grid::scan(input)
        .filter(|&(_, c)| c == '@')
        .map(|(v, _)| v)
        .collect()
}

fn accessible(rolls: &HashSet<Vector>, roll: &Vector) -> bool {
    roll.adjacent8().filter(|a| rolls.contains(a)).count() < 4
}

pub fn part1(input: &str) -> usize {
    let rolls = parse(input);
    rolls.iter().filter(|&r| accessible(&rolls, r)).count()
}

pub fn part2(input: &str) -> usize {
    let mut rolls = parse(input);
    let mut stack: Vec<Vector> = rolls.iter().copied().collect();

    let mut removed = 0;
    while let Some(roll) = stack.pop() {
        if rolls.contains(&roll) && accessible(&rolls, &roll) {
            rolls.remove(&roll);
            removed += 1;
            stack.extend(roll.adjacent8().filter(|r| rolls.contains(r)));
        }
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
