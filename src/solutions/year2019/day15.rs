use crate::{
    grid::{E, N, S, Vector, W, Z},
    intcode::VM,
    search,
};

#[derive(Clone)]
struct State {
    vm: VM,
    pos: Vector,
    steps: u32,
    found_oxygen_system: bool,
}

impl State {
    fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        [N, S, W, E].into_iter().enumerate().map(|(i, dir)| {
            let mut state = self.clone();
            state.vm.input(i64::try_from(i).unwrap() + 1);
            match state.vm.output() {
                0 => {}
                1 => {
                    state.pos += dir;
                    state.steps += 1;
                }
                2 => {
                    state.pos += dir;
                    state.steps += 1;
                    state.found_oxygen_system = true;
                }
                _ => unreachable!(),
            }
            state
        })
    }
}

fn oxygen_system(input: &str) -> State {
    search::breadth_first(
        State {
            vm: VM::new(input),
            pos: Z,
            steps: 0,
            found_oxygen_system: false,
        },
        |state, push| state.adjacent().for_each(push),
        search::hash_filter(|state: &State| state.pos),
    )
    .find(|state| state.found_oxygen_system)
    .unwrap()
}

pub fn part1(input: &str) -> u32 {
    oxygen_system(input).steps
}

pub fn part2(input: &str) -> u32 {
    search::breadth_first(
        State {
            steps: 0,
            ..oxygen_system(input)
        },
        |state, push| state.adjacent().for_each(push),
        search::hash_filter(|state: &State| state.pos),
    )
    .map(|state| state.steps)
    .max()
    .unwrap()
}
