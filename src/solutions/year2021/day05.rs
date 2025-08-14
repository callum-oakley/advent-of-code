use crate::{
    freqs::Freqs,
    grid::{IntoVector, Vector},
};

fn parse(input: &str) -> impl Iterator<Item = (Vector, Vector)> + '_ {
    input.trim().lines().map(|line| {
        let (s, t) = line.split_once("->").unwrap();
        (s.into_vector(), t.into_vector())
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|&(s, t)| s.x == t.x || s.y == t.y)
        .flat_map(|(s, t)| crate::grid::line_segment(s, t))
        .freqs()
        .values()
        .filter(|&&f| f >= 2)
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .flat_map(|(s, t)| crate::grid::line_segment(s, t))
        .freqs()
        .values()
        .filter(|&&f| f >= 2)
        .count()
}

pub fn tests() {
    let example = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    ";
    assert_eq!(part1(example), 5);
    assert_eq!(part2(example), 12);
}
