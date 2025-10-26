use std::{cmp, collections::HashMap};

use crate::part::Part;

struct Instruction<'a> {
    reg: &'a str,
    op: &'a str,
    diff: i32,
    cond_lhs: &'a str,
    cond_op: &'a str,
    cond_rhs: i32,
}

fn parse(input: &str) -> impl Iterator<Item = Instruction<'_>> + '_ {
    input.lines().map(|line| {
        let mut words = line.split_whitespace();
        Instruction {
            reg: words.next().unwrap(),
            op: words.next().unwrap(),
            diff: words.next().unwrap().parse().unwrap(),
            cond_lhs: words.nth(1).unwrap(),
            cond_op: words.next().unwrap(),
            cond_rhs: words.next().unwrap().parse().unwrap(),
        }
    })
}

fn part_(part: Part, input: &str) -> i32 {
    let mut regs = HashMap::new();
    let mut max_val = 0;
    for instruction in parse(input) {
        let lhs = regs.get(instruction.cond_lhs).unwrap_or(&0);
        let op = match instruction.cond_op {
            ">" => i32::gt,
            "<" => i32::lt,
            ">=" => i32::ge,
            "<=" => i32::le,
            "==" => i32::eq,
            "!=" => i32::ne,
            _ => unreachable!(),
        };
        if op(lhs, &instruction.cond_rhs) {
            match instruction.op {
                "inc" => *regs.entry(instruction.reg).or_insert(0) += instruction.diff,
                "dec" => *regs.entry(instruction.reg).or_insert(0) -= instruction.diff,
                _ => unreachable!(),
            }
            max_val = cmp::max(max_val, regs[instruction.reg]);
        }
    }
    match part {
        Part::One => *regs.values().max().unwrap(),
        Part::Two => max_val,
    }
}

pub fn part1(input: &str) -> i32 {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> i32 {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "b inc 5 if a > 1
                   a inc 1 if b < 5
                   c dec -10 if a >= 1
                   c inc -20 if c == 10";
    assert_eq!(part1(example), 1);
    assert_eq!(part2(example), 10);
}
