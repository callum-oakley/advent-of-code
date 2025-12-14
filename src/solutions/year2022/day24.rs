use std::collections::HashSet;

use crate::grid::{Adjacent, Grid, NW, Vector};

struct Blizzard {
    pos: Vector,
    dir: Vector,
}

fn parse(input: &str) -> (HashSet<Vector>, Vec<Blizzard>, Vector, Vector, Vector) {
    let g = Grid::parse(input, |_, c| c);

    let mut valley = HashSet::new();
    let mut blizzards = Vec::new();
    for (v, &c) in &g {
        let pos = v + NW;

        if "^>v<".contains(c) {
            blizzards.push(Blizzard {
                pos,
                dir: crate::cast::char_to_vector(c),
            });
        }

        if c != '#' {
            valley.insert(pos);
        }
    }

    let period = g.size + 2 * NW;
    let start = *valley.iter().min_by_key(|v| v.y).unwrap();
    let goal = *valley.iter().max_by_key(|v| v.y).unwrap();

    (valley, blizzards, period, start, goal)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Vector,
    minutes: i64,
}

fn search(
    valley: &HashSet<Vector>,
    blizzards: &[Blizzard],
    period: Vector,
    state: State,
    goal: Vector,
) -> State {
    crate::search::a_star(
        state,
        |state, push| {
            let m = state.minutes + 1;
            for v in state.pos.adjacent5() {
                if valley.contains(&v)
                    && !blizzards
                        .iter()
                        .any(|b| (b.pos + b.dir * m).zip_map(&period, i64::rem_euclid) == v)
                {
                    push(State { pos: v, minutes: m });
                }
            }
        },
        crate::search::id_filter(),
        |state| state.minutes,
        |state| (goal - state.pos).abs().sum(),
    )
    .find(|s| s.pos == goal)
    .unwrap()
}

pub fn part1(input: &str) -> i64 {
    let (valley, blizzards, period, start, goal) = parse(input);
    let state = State {
        pos: start,
        minutes: 0,
    };
    search(&valley, &blizzards, period, state, goal).minutes
}

pub fn part2(input: &str) -> i64 {
    let (valley, blizzards, period, start, goal) = parse(input);
    let state = State {
        pos: start,
        minutes: 0,
    };
    let state = search(&valley, &blizzards, period, state, goal);
    let state = search(&valley, &blizzards, period, state, start);
    let state = search(&valley, &blizzards, period, state, goal);
    state.minutes
}

pub fn tests() {
    let example = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#";
    assert_eq!(part1(example), 18);
    assert_eq!(part2(example), 54);
}
