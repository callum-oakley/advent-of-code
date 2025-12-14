use std::ops::Div;

use crate::{
    grid::{Adjacent, Grid, NW, Vector, Z},
    search,
};

fn search(cave: &Grid<u32>) -> u32 {
    struct State {
        pos: Vector,
        cost: u32,
    }
    search::dijkstra(
        State { pos: Z, cost: 0 },
        |s, push| {
            s.pos
                .adjacent4()
                .filter_map(|a| {
                    cave.get(a).map(|risk| State {
                        pos: a,
                        cost: s.cost + risk,
                    })
                })
                .for_each(push);
        },
        search::hash_filter(|s: &State| s.pos),
        |s| s.cost,
    )
    .find(|s| s.pos == cave.size + NW)
    .unwrap()
    .cost
}

pub fn part1(input: &str) -> u32 {
    search(&Grid::parse(input, |_, c| c.to_digit(10).unwrap()))
}

pub fn part2(input: &str) -> u32 {
    let small_cave = Grid::parse(input, |_, c| c.to_digit(10).unwrap());
    let mut big_cave = Grid::new(0, small_cave.size * 5);
    for v in big_cave.keys() {
        big_cave[v] = crate::number_theory::wrap(
            small_cave[v.zip_map(&small_cave.size, i64::rem_euclid)]
                + u32::try_from(v.zip_map(&small_cave.size, Div::div).abs().sum()).unwrap(),
            1,
            10,
        );
    }
    search(&big_cave)
}

pub fn tests() {
    let example = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ]
    .join("\n");
    assert_eq!(part1(&example), 40);
    assert_eq!(part2(&example), 315);
}
