use std::collections::VecDeque;

use regex::Regex;

use crate::part::Part;

struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    Regex::new(concat!(
        r"Monkey \d+:\s*",
        r"Starting items: (\d+(?:, \d+)*)\s*",
        r"Operation: new = old (\+|\*) (old|\d+)\s*",
        r"Test: divisible by (\d+)\s*",
        r"If true: throw to monkey (\d+)\s*",
        r"If false: throw to monkey (\d+)\s*",
    ))
    .unwrap()
    .captures_iter(input)
    .map(|captures| Monkey {
        items: captures[1]
            .split(", ")
            .map(|w| w.parse().unwrap())
            .collect(),
        operation: {
            if &captures[2] == "*" && &captures[3] == "old" {
                Box::new(|old| old * old)
            } else {
                let a: usize = captures[3].parse().unwrap();
                match &captures[2] {
                    "+" => Box::new(move |old| old + a),
                    "*" => Box::new(move |old| old * a),
                    _ => unreachable!(),
                }
            }
        },
        test: captures[4].parse().unwrap(),
        if_true: captures[5].parse().unwrap(),
        if_false: captures[6].parse().unwrap(),
        inspections: 0,
    })
    .collect()
}

fn part_(part: Part, input: &str) -> usize {
    let rounds = match part {
        Part::One => 20,
        Part::Two => 10000,
    };

    let mut monkeys = parse(input);

    // We only care about whether worry levels are divisible by our tests, so we can take the
    // modulus with the LCM of all the tests at each step.
    let lcm = monkeys
        .iter()
        .map(|m| m.test)
        .reduce(num::integer::lcm)
        .unwrap();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspections += 1;

                let mut item = (monkeys[i].operation)(item);
                match part {
                    Part::One => item /= 3,
                    Part::Two => item %= lcm,
                }

                let j = if item % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[j].items.push_back(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| std::cmp::Reverse(m.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    ";
    assert_eq!(part1(example), 10605);
    assert_eq!(part2(example), 2_713_310_158);
}
