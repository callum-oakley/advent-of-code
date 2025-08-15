use crate::{
    grid::{Adjacent, Grid, Vector},
    search,
};

fn parse(input: &str) -> (Grid<bool>, Vector) {
    let mut start = None;
    let garden = Grid::parse(input, |pos, c| match c {
        'S' => {
            start = Some(pos);
            true
        }
        '.' => true,
        '#' => false,
        _ => unreachable!(),
    });
    (garden, start.unwrap())
}

fn part_(garden: &Grid<bool>, start: Vector, max_steps: usize) -> usize {
    struct State {
        pos: Vector,
        steps: usize,
    }
    search::breadth_first(
        State {
            pos: start,
            steps: 0,
        },
        |state, push| {
            if state.steps < max_steps {
                for pos in state.pos.adjacent4() {
                    if garden[pos.zip_map(&garden.size, i64::rem_euclid)] {
                        push(State {
                            pos,
                            steps: state.steps + 1,
                        });
                    }
                }
            }
        },
        search::hash_filter(|state: &State| state.pos),
    )
    .filter(|state| state.steps % 2 == max_steps % 2)
    .count()
}

// https://en.wikipedia.org/wiki/Lagrange_polynomial
fn lagrange(
    (x0, y0): (usize, usize),
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) -> impl Fn(usize) -> usize {
    move |x| {
        y0 * (x - x1) * (x - x2) / ((x1 - x0) * (x2 - x0))
            - y1 * (x - x0) * (x - x2) / ((x2 - x1) * (x1 - x0))
            + y2 * (x - x0) * (x - x1) / ((x2 - x0) * (x2 - x1))
    }
}

pub fn part1(input: &str) -> usize {
    let (garden, start) = parse(input);
    part_(&garden, start, 64)
}

pub fn part2(input: &str) -> usize {
    let (garden, start) = parse(input);
    let y0 = part_(&garden, start, 65);
    let y1 = part_(&garden, start, 65 + 131);
    let y2 = part_(&garden, start, 65 + 2 * 131);

    let p = lagrange((0, y0), (1, y1), (2, y2));

    assert_eq!(p(3), part_(&garden, start, 65 + 3 * 131));

    // 65 + 202_300 * 131 == 26_501_365
    p(202_300)
}

pub fn tests() {
    let example = [
        "...........",
        ".....###.#.",
        ".###.##..#.",
        "..#.#...#..",
        "....#.#....",
        ".##..S####.",
        ".##..#...#.",
        ".......##..",
        ".##.#.####.",
        ".##..##.##.",
        "...........",
    ]
    .join("\n");
    let (garden, start) = parse(&example);
    assert_eq!(part_(&garden, start, 6), 16);
    assert_eq!(part_(&garden, start, 10), 50);
    assert_eq!(part_(&garden, start, 50), 1594);
    assert_eq!(part_(&garden, start, 100), 6536);
    assert_eq!(part_(&garden, start, 500), 167_004);
    assert_eq!(part_(&garden, start, 1000), 668_697);
    assert_eq!(part_(&garden, start, 5000), 16_733_044);
}
