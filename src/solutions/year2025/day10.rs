use std::{collections::BTreeSet, sync::LazyLock};

use regex::Regex;
use z3::{Optimize, SatResult, ast::Int};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(.*)\] (.*) \{(.*)\}").unwrap());
    RE.captures_iter(input).map(|captures| Machine {
        lights: captures[1].chars().map(|c| c == '#').collect(),
        buttons: captures[2]
            .split_whitespace()
            .map(|b| crate::cast::str_to_ints(b).collect())
            .collect(),
        joltages: crate::cast::str_to_ints(&captures[3]).collect(),
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .map(|machine| {
            crate::search::breadth_first(
                BTreeSet::new(),
                |state, push| {
                    for b in machine.buttons.iter().filter(|b| !state.contains(b)) {
                        let mut s = state.clone();
                        s.insert(b);
                        push(s);
                    }
                },
                crate::search::id_filter(),
            )
            .find(|state| {
                let mut lights = vec![false; machine.lights.len()];
                for &button in state {
                    for &i in button {
                        lights[i] = !lights[i];
                    }
                }
                lights == machine.lights
            })
            .unwrap()
            .len()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input)
        .map(|machine| {
            let optimize = Optimize::new();

            // Let b{i} be the number of presses for button i.
            let buttons: Vec<Int> = (0..machine.buttons.len())
                .map(|i| Int::new_const(format!("b{i}")))
                .collect();

            // For all i b{i} >= 0.
            for button in &buttons {
                optimize.assert(&button.ge(0));
            }

            // For each joltage the sum of presses for connected buttons equals that joltage.
            for (i, &joltage) in machine.joltages.iter().enumerate() {
                optimize.assert(
                    &machine
                        .buttons
                        .iter()
                        .enumerate()
                        .filter(|(_, button)| button.contains(&i))
                        .map(|(j, _)| &buttons[j])
                        .sum::<Int>()
                        .eq(joltage),
                );
            }

            // Minimize total button presses.
            let presses = buttons.iter().sum::<Int>();
            optimize.minimize(&presses);

            assert_eq!(optimize.check(&[]), SatResult::Sat);
            optimize
                .get_model()
                .unwrap()
                .eval(&presses, false)
                .unwrap()
                .as_u64()
                .unwrap()
        })
        .sum()
}

pub fn tests() {
    let example = "
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";
    assert_eq!(part1(example), 7);
    assert_eq!(part2(example), 33);
}
