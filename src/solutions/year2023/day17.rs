use crate::{
    grid::{E, Grid, LEFT, NW, RIGHT, Turn, Vector, Z},
    search,
};

#[derive(Clone)]
struct Crucible {
    pos: Vector,
    dir: Vector,
    straight_len: u8,
    heat_loss: u32,
}

impl Crucible {
    fn step(&self, city: &Grid<u32>, turn: Option<Turn>) -> Option<Self> {
        let mut crucible = self.clone();
        if let Some(turn) = turn {
            crucible.dir = turn * crucible.dir;
            crucible.straight_len = 1;
        } else {
            crucible.straight_len += 1;
        }
        crucible.pos += crucible.dir;
        crucible.heat_loss += city.get(crucible.pos)?;
        Some(crucible)
    }
}

fn part_(min_straight_len: u8, max_straight_len: u8, input: &str) -> u32 {
    let city = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    let target = city.size + NW;

    search::dijkstra(
        Crucible {
            pos: Z,
            dir: E,
            straight_len: 0,
            heat_loss: 0,
        },
        move |crucible, push| {
            if crucible.straight_len >= min_straight_len {
                if let Some(crucible) = crucible.step(&city, Some(LEFT)) {
                    push(crucible);
                }
                if let Some(crucible) = crucible.step(&city, Some(RIGHT)) {
                    push(crucible);
                }
            }
            if crucible.straight_len < max_straight_len
                && let Some(crucible) = crucible.step(&city, None)
            {
                push(crucible);
            }
        },
        search::hash_filter(|crucible: &Crucible| {
            (crucible.pos, crucible.dir, crucible.straight_len)
        }),
        |crucible| crucible.heat_loss,
    )
    .find(|crucible| crucible.pos == target && crucible.straight_len >= min_straight_len)
    .unwrap()
    .heat_loss
}

pub fn part1(input: &str) -> u32 {
    part_(0, 3, input)
}

pub fn part2(input: &str) -> u32 {
    part_(4, 10, input)
}

pub fn tests() {
    let example0 = [
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ]
    .join("\n");
    let example1 = [
        "111111111111",
        "999999999991",
        "999999999991",
        "999999999991",
        "999999999991",
    ]
    .join("\n");
    assert_eq!(part1(&example0), 102);
    assert_eq!(part2(&example0), 94);
    assert_eq!(part2(&example1), 71);
}
