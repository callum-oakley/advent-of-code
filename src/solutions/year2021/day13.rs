use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;

use crate::grid::Vector;

#[derive(Clone, Copy)]
struct Fold {
    axis: usize,
    offset: i64,
}

fn parse(input: &str) -> (HashSet<Vector>, impl Iterator<Item = Fold> + '_) {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"fold along (x|y)=(\d+)").unwrap());
    let (dots, folds) = input.split_once("\n\n").unwrap();
    (
        dots.split_whitespace()
            .map(crate::cast::str_to_vector)
            .collect(),
        RE.captures_iter(folds).map(|captures| Fold {
            axis: match &captures[1] {
                "x" => 0,
                "y" => 1,
                _ => unreachable!(),
            },
            offset: captures[2].parse().unwrap(),
        }),
    )
}

fn apply_fold(dots: &HashSet<Vector>, fold: Fold) -> HashSet<Vector> {
    dots.iter()
        .map(|&(mut dot)| {
            if dot[fold.axis] > fold.offset {
                dot[fold.axis] = 2 * fold.offset - dot[fold.axis];
            }
            dot
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let (dots, mut folds) = parse(input);
    apply_fold(&dots, folds.next().unwrap()).len()
}

pub fn part2(input: &str) -> &str {
    let (dots, folds) = parse(input);
    crate::ocr::parse(&crate::cast::vector_hash_set_to_string(
        &folds.fold(dots, |d, f| apply_fold(&d, f)),
    ))
}

pub fn tests() {
    let example = "
        6,10 0,14 9,10 0,3 10,4 4,11 6,0 6,12 4,1 0,13 10,12 3,4 3,0 8,4 1,10 2,14 8,10 9,0

        fold along y=7
        fold along x=5
    ";
    assert_eq!(part1(example), 17);
}
