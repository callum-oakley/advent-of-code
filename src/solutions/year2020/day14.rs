use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use regex::Regex;

enum Instruction<'a> {
    Mask(&'a str),
    Mem(u64, u64),
}

fn parse(input: &str) -> impl Iterator<Item = Instruction<'_>> {
    static MASK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mask = ([X01]+)").unwrap());
    static MEM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap());
    input.trim().lines().map(|line| {
        if let Some(captures) = MASK.captures(line) {
            Instruction::Mask(captures.get(1).unwrap().as_str())
        } else if let Some(captures) = MEM.captures(line) {
            let i: u64 = captures[1].parse().unwrap();
            let v: u64 = captures[2].parse().unwrap();
            Instruction::Mem(i, v)
        } else {
            unreachable!()
        }
    })
}

pub fn part1(input: &str) -> u64 {
    let mut mask = "";
    let mut mem = HashMap::new();
    for instruction in parse(input) {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(addr, mut v) => {
                for (i, d) in mask.chars().rev().enumerate() {
                    match d {
                        '0' => v &= !(1 << i),
                        '1' => v |= 1 << i,
                        _ => (),
                    }
                }
                mem.insert(addr, v % (1 << 36));
            }
        }
    }
    mem.values().sum()
}

pub fn part2(input: &str) -> u64 {
    let mut mask = "";
    let mut mem = HashMap::new();
    for instruction in parse(input) {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(addr, v) => {
                let mut addrs = HashSet::from([addr]);
                for (i, d) in mask.chars().rev().enumerate() {
                    match d {
                        '1' => addrs = addrs.into_iter().map(|addr| addr | (1 << i)).collect(),
                        'X' => {
                            addrs = addrs
                                .into_iter()
                                .flat_map(|addr| [addr, addr ^ (1 << i)])
                                .collect();
                        }
                        _ => (),
                    }
                }
                for addr in addrs {
                    mem.insert(addr, v % (1 << 36));
                }
            }
        }
    }
    mem.values().sum()
}

pub fn tests() {
    let example1 = "
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
    ";
    let example2 = "
        mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1
    ";
    assert_eq!(part1(example1), 165);
    assert_eq!(part2(example2), 208);
}
