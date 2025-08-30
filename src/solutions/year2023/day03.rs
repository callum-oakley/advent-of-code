use std::collections::{HashMap, HashSet};

use nalgebra::vector;
use regex::Regex;

use crate::grid::{self, Adjacent, Vector};

struct Label {
    val: u32,
    covers: HashSet<Vector>,
}

fn parse(input: &str) -> (HashMap<Vector, char>, Vec<Label>) {
    let num = Regex::new(r"\d+").unwrap();

    let symbols = grid::scan(input)
        .filter(|&(_, c)| c != '.' && !c.is_ascii_digit())
        .collect();

    let labels = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = i64::try_from(y).unwrap();
            num.find_iter(line).map(move |m| Label {
                val: m.as_str().parse().unwrap(),
                covers: (m.start()..m.end())
                    .flat_map(|x| {
                        let x = i64::try_from(x).unwrap();
                        vector![x, y].adjacent8()
                    })
                    .collect(),
            })
        })
        .collect();

    (symbols, labels)
}

pub fn part1(input: &str) -> u32 {
    let (symbols, labels) = parse(input);
    labels
        .iter()
        .filter(|label| label.covers.iter().any(|pos| symbols.contains_key(pos)))
        .map(|label| label.val)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (symbols, labels) = parse(input);
    symbols
        .iter()
        .filter(|&(_, &c)| c == '*')
        .filter_map(|(pos, _)| {
            let mut adj = labels.iter().filter(|label| label.covers.contains(pos));
            if let (Some(a), Some(b), None) = (adj.next(), adj.next(), adj.next()) {
                Some(a.val * b.val)
            } else {
                None
            }
        })
        .sum()
}

pub fn tests() {
    let example = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ]
    .join("\n");

    assert_eq!(part1(&example), 4361);
    assert_eq!(part2(&example), 467_835);
}
