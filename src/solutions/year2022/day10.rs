use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;

use crate::grid::{Grid, Vector};

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"noop|addx (-?\d+)").unwrap());
    RE.captures_iter(input).map(|captures| {
        if &captures[0] == "noop" {
            Instruction::Noop
        } else {
            Instruction::Addx(captures[1].parse().unwrap())
        }
    })
}

fn cpu(input: &str) -> Vec<i32> {
    let mut res = vec![1];
    for instruction in parse(input) {
        res.push(res[res.len() - 1]);
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(y) => {
                res.push(res[res.len() - 1] + y);
            }
        }
    }
    res
}

pub fn part1(input: &str) -> i32 {
    let xs = cpu(input);
    (0..)
        .map(|i| 20 + 40 * i)
        .map_while(|cycle| xs.get(cycle - 1).map(|x| i32::try_from(cycle).unwrap() * x))
        .sum()
}

fn part2_(input: &str) -> HashSet<Vector> {
    let xs = cpu(input);
    let mut pixels = HashSet::new();
    for x in 0i32..40 {
        for y in 0i32..6 {
            if (xs[usize::try_from(40 * y + x).unwrap()] - x).abs() <= 1 {
                pixels.insert(nalgebra::vector![x, y]);
            }
        }
    }
    pixels
}

pub fn part2(input: &str) -> &str {
    crate::ocr::parse(part2_(input))
}

pub fn tests() {
    let example = "
        addx 15 addx -11 addx 6 addx -3 addx 5 addx -1 addx -8 addx 13 addx 4 noop addx -1 addx 5
        addx -1 addx 5 addx -1 addx 5 addx -1 addx 5 addx -1 addx -35 addx 1 addx 24 addx -19 addx 1
        addx 16 addx -11 noop noop addx 21 addx -15 noop noop addx -3 addx 9 addx 1 addx -3 addx 8
        addx 1 addx 5 noop noop noop noop noop addx -36 noop addx 1 addx 7 noop noop noop addx 2
        addx 6 noop noop noop noop noop addx 1 noop noop addx 7 addx 1 noop addx -13 addx 13 addx 7
        noop addx 1 addx -33 noop noop noop addx 2 noop noop noop addx 8 noop addx -1 addx 2 addx 1
        noop addx 17 addx -9 addx 1 addx 1 addx -3 addx 11 noop noop addx 1 noop addx 1 noop noop
        addx -13 addx -19 addx 1 addx 3 addx 26 addx -30 addx 12 addx -1 addx 3 addx 1 noop noop
        noop addx -9 addx 18 addx 1 addx 2 noop noop addx 9 noop noop noop addx -1 addx 2 addx -37
        addx 1 addx 3 noop addx 15 addx -21 addx 22 addx -6 addx 1 noop addx 2 addx 1 noop addx -10
        noop noop addx 20 addx 1 addx 2 addx 2 addx -6 addx -11 noop noop noop
    ";
    assert_eq!(part1(example), 13140);

    let image = concat!(
        "##..##..##..##..##..##..##..##..##..##..\n",
        "###...###...###...###...###...###...###.\n",
        "####....####....####....####....####....\n",
        "#####.....#####.....#####.....#####.....\n",
        "######......######......######......####\n",
        "#######.......#######.......#######.....\n",
    );
    assert_eq!(Grid::from(part2_(example)).to_string(), image);
}
