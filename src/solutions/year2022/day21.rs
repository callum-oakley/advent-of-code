use std::collections::HashMap;

use num::Rational64;

enum Job<'a> {
    Num(Rational64),
    Op(
        &'static dyn Fn(Rational64, Rational64) -> Rational64,
        &'a str,
        &'a str,
    ),
}

fn parse(input: &str) -> HashMap<&str, Job<'_>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.trim().split_once(": ").unwrap();
            let rhs: Vec<_> = rhs.split_whitespace().collect();
            if rhs.len() == 3 {
                let op: &'static dyn Fn(Rational64, Rational64) -> Rational64 = match rhs.get(1) {
                    Some(&"+") => &|a, b| a + b,
                    Some(&"-") => &|a, b| a - b,
                    Some(&"*") => &|a, b| a * b,
                    Some(&"/") => &|a, b| a / b,
                    _ => unreachable!(),
                };
                (lhs, Job::Op(op, rhs[0], rhs[2]))
            } else {
                (lhs, Job::Num(rhs[0].parse().unwrap()))
            }
        })
        .collect()
}

fn eval(monkeys: &HashMap<&str, Job>, monkey: &str) -> Rational64 {
    match monkeys[monkey] {
        Job::Num(num) => num,
        Job::Op(op, a, b) => op(eval(monkeys, a), eval(monkeys, b)),
    }
}

pub fn part1(input: &str) -> Rational64 {
    eval(&parse(input), "root")
}

pub fn part2(input: &str) -> Rational64 {
    let mut monkeys = parse(input);
    let Job::Op(op, _, _) = monkeys.get_mut("root").unwrap() else {
        unreachable!();
    };
    *op = &|a, b| a - b;

    let mut f = |humn| {
        monkeys.insert("humn", Job::Num(humn));
        eval(&monkeys, "root")
    };

    // Since monkeys is a tree, f is a linear function of humn, so we can find the root by taking
    // the value of f at two points and extrapolating.
    f(0.into()) / (f(0.into()) - f(1.into()))
}

pub fn tests() {
    let example = "
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    ";
    assert_eq!(part1(example), 152.into());
    assert_eq!(part2(example), 301.into());
}
