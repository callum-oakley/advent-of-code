use crate::grid::{Adjacent, Grid, Vector};

struct State {
    pos: Vector,
    steps: usize,
}

fn parse(input: &str) -> (Grid<u8>, Vector, Vector) {
    let mut start = None;
    let mut target = None;
    let g = Grid::parse(input, |v, c| match c {
        'S' => {
            start = Some(v);
            b'a'
        }
        'E' => {
            target = Some(v);
            b'z'
        }
        _ => u8::try_from(c).unwrap(),
    });
    (g, start.unwrap(), target.unwrap())
}

// Searching from the target back to the start means we can use the same search for part 2, but find
// the first 'a' instead.
fn search_from_target(g: &Grid<u8>, target: Vector) -> impl Iterator<Item = State> {
    crate::search::breadth_first(
        State {
            pos: target,
            steps: 0,
        },
        |state, push| {
            for v in state.pos.adjacent4() {
                if g.get(v).is_some_and(|&h| g[state.pos] <= h + 1) {
                    push(State {
                        pos: v,
                        steps: state.steps + 1,
                    });
                }
            }
        },
        crate::search::hash_filter(|state: &State| state.pos),
    )
}

pub fn part1(input: &str) -> usize {
    let (g, start, target) = parse(input);
    search_from_target(&g, target)
        .find(|state| state.pos == start)
        .unwrap()
        .steps
}

pub fn part2(input: &str) -> usize {
    let (g, _, target) = parse(input);
    search_from_target(&g, target)
        .find(|state| g[state.pos] == b'a')
        .unwrap()
        .steps
}

pub fn tests() {
    let example = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
    assert_eq!(part1(example), 31);
    assert_eq!(part2(example), 29);
}
