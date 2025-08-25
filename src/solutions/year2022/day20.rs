use crate::part::Part;

struct State {
    values: Vec<i64>,
    next: Vec<usize>,
    prev: Vec<usize>,
}

impl State {
    fn iter(&self) -> impl Iterator<Item = i64> {
        let mut i = 0;
        std::iter::from_fn(move || {
            let res = self.values[i];
            i = self.next[i];
            Some(res)
        })
    }
}

fn parse(input: &str) -> State {
    let values: Vec<_> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let next = (0..values.len()).map(|i| (i + 1) % values.len()).collect();
    let prev = (0..values.len())
        .map(|i| (i + values.len() - 1) % values.len())
        .collect();
    State { values, next, prev }
}

fn part_(part: Part, input: &str) -> i64 {
    let mut state = parse(input);

    let rounds = match part {
        Part::One => 1,
        Part::Two => 10,
    };

    if part == Part::Two {
        for value in &mut state.values {
            *value *= 811_589_153;
        }
    }

    for _ in 0..rounds {
        for i in 0..state.values.len() {
            let n = state.next[i];
            let mut p = state.prev[i];
            state.next[p] = n;
            state.prev[n] = p;

            // NOTE we're taking the mod with state.values.len() - 1 as at this point we've removed
            // a value from the ring.
            let dist = usize::try_from(
                state.values[i].rem_euclid(i64::try_from(state.values.len() - 1).unwrap()),
            )
            .unwrap();
            for _ in 0..dist {
                p = state.next[p];
            }
            let n = state.next[p];

            state.next[p] = i;
            state.prev[i] = p;
            state.next[i] = n;
            state.prev[n] = i;
        }
    }

    state
        .iter()
        .skip_while(|&v| v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

pub fn part1(input: &str) -> i64 {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> i64 {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "1 2 -3 3 -2 0 4";
    assert_eq!(part1(example), 3);
    assert_eq!(part2(example), 1_623_178_306);
}
