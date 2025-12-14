use crate::{
    grid::{Adjacent, Vector},
    search,
};

fn is_open(seed: u32, v: Vector) -> bool {
    (u32::try_from(v.x * v.x + 3 * v.x + 2 * v.x * v.y + v.y + v.y * v.y).unwrap() + seed)
        .count_ones()
        .is_multiple_of(2)
}

#[derive(Clone)]
struct State {
    pos: Vector,
    steps: u32,
}

fn search(input: &str) -> impl Iterator<Item = State> {
    let seed = input.parse().unwrap();
    search::breadth_first(
        State {
            pos: Vector::new(1, 1),
            steps: 0,
        },
        move |state, push| {
            for pos in state.pos.adjacent4() {
                if pos.x >= 0 && pos.y >= 0 && is_open(seed, pos) {
                    push(State {
                        pos,
                        steps: state.steps + 1,
                    });
                }
            }
        },
        search::hash_filter(|state: &State| state.pos),
    )
}

fn part1_(target: Vector, input: &str) -> u32 {
    search(input)
        .find(|state| state.pos == target)
        .unwrap()
        .steps
}

pub fn part1(input: &str) -> u32 {
    part1_(Vector::new(31, 39), input)
}

pub fn part2(input: &str) -> usize {
    search(input).take_while(|state| state.steps <= 50).count()
}

pub fn tests() {
    assert_eq!(part1_(Vector::new(7, 4), "10"), 11);
}
