use std::{cell::RefCell, collections::HashMap};

use crate::{
    grid::{Adjacent, IntoVector, Vector, N, W, Z},
    search,
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

const TOOLS: [Tool; 3] = [Tool::Torch, Tool::ClimbingGear, Tool::Neither];

fn compatible(tile: Tile, tool: Tool) -> bool {
    match tile {
        Tile::Rocky => tool != Tool::Neither,
        Tile::Wet => tool != Tool::Torch,
        Tile::Narrow => tool != Tool::ClimbingGear,
    }
}

impl From<i64> for Tile {
    fn from(erosion: i64) -> Self {
        match erosion % 3 {
            0 => Tile::Rocky,
            1 => Tile::Wet,
            2 => Tile::Narrow,
            _ => unreachable!(),
        }
    }
}

struct Cave {
    depth: i64,
    target: Vector,
    erosion_cache: RefCell<HashMap<Vector, i64>>,
}

impl Cave {
    fn erosion(&self, pos: Vector) -> i64 {
        if let Some(&res) = self.erosion_cache.borrow().get(&pos) {
            return res;
        }
        let geo_index = if pos == Z || pos == self.target {
            0
        } else if pos.y == 0 {
            pos.x * 16807
        } else if pos.x == 0 {
            pos.y * 48271
        } else {
            self.erosion(pos + W) * self.erosion(pos + N)
        };
        let res = (geo_index + self.depth) % 20183;
        self.erosion_cache.borrow_mut().insert(pos, res);
        res
    }
}

fn parse(input: &str) -> Cave {
    let (depth, target) = input.split_once('\n').unwrap();
    Cave {
        depth: depth.strip_prefix("depth: ").unwrap().parse().unwrap(),
        target: target.strip_prefix("target: ").unwrap().into_vector(),
        erosion_cache: RefCell::new(HashMap::new()),
    }
}

#[derive(Clone)]
struct State {
    pos: Vector,
    tool: Tool,
    mins: i64,
}

impl State {
    fn adjacent<'a>(&'a self, cave: &'a Cave) -> impl Iterator<Item = Self> + 'a {
        self.pos
            .adjacent4()
            .filter_map(|pos| {
                if pos.x >= 0 && pos.y >= 0 && compatible(cave.erosion(pos).into(), self.tool) {
                    Some(State {
                        pos,
                        mins: self.mins + 1,
                        ..self.clone()
                    })
                } else {
                    None
                }
            })
            .chain(TOOLS.iter().filter_map(|&tool| {
                if tool != self.tool && compatible(cave.erosion(self.pos).into(), tool) {
                    Some(State {
                        tool,
                        mins: self.mins + 7,
                        ..self.clone()
                    })
                } else {
                    None
                }
            }))
    }
}

pub fn part1(input: &str) -> i64 {
    let cave = parse(input);
    let mut res = 0;
    for y in 0..=cave.target.y {
        for x in 0..=cave.target.x {
            res += cave.erosion(Vector::new(x, y)) % 3;
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let cave = parse(input);
    let target = cave.target;
    search::a_star(
        State {
            pos: Z,
            tool: Tool::Torch,
            mins: 0,
        },
        move |state, push| state.adjacent(&cave).for_each(push),
        search::hash_filter(|state: &State| (state.pos, state.tool)),
        |state| state.mins,
        move |state| (target - state.pos).abs().sum(),
    )
    .find(|state| state.pos == target && state.tool == Tool::Torch)
    .unwrap()
    .mins
}

pub fn tests() {
    let example = "depth: 510\ntarget: 10,10";
    assert_eq!(parse(example).erosion(Vector::new(1, 1)), 1805);
    assert_eq!(part1(example), 114);
    assert_eq!(part2(example), 45);
}
