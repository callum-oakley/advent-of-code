use std::{cmp::Ordering, sync::LazyLock};

use crate::grid::{Grid, Vector};

struct Robot {
    p: Vector,
    v: Vector,
}

static TREE: LazyLock<Grid<bool>> = LazyLock::new(|| {
    Grid::parse(
        &[
            "###############################",
            "#.............................#",
            "#.............................#",
            "#.............................#",
            "#.............................#",
            "#..............#..............#",
            "#.............###.............#",
            "#............#####............#",
            "#...........#######...........#",
            "#..........#########..........#",
            "#............#####............#",
            "#...........#######...........#",
            "#..........#########..........#",
            "#.........###########.........#",
            "#........#############........#",
            "#..........#########..........#",
            "#.........###########.........#",
            "#........#############........#",
            "#.......###############.......#",
            "#......#################......#",
            "#........#############........#",
            "#.......###############.......#",
            "#......#################......#",
            "#.....###################.....#",
            "#....#####################....#",
            "#.............###.............#",
            "#.............###.............#",
            "#.............###.............#",
            "#.............................#",
            "#.............................#",
            "#.............................#",
            "#.............................#",
            "###############################",
        ]
        .join("\n"),
        |_, c| c == '#',
    )
});

fn parse(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (p, v) = line.trim().split_once(' ').unwrap();
            Robot {
                p: crate::cast::string_to_vector(p),
                v: crate::cast::string_to_vector(v),
            }
        })
        .collect()
}

fn tick(space: Vector, robots: &mut [Robot]) {
    for robot in robots {
        robot.p += robot.v;
        robot.p = robot.p.zip_map(&space, |p, s| p.rem_euclid(s));
    }
}

fn quadrants(space: Vector, robots: &[Robot]) -> [usize; 4] {
    let mut res = [0; 4];
    for robot in robots {
        match (
            robot.p.x.cmp(&((space.x - 1) / 2)),
            robot.p.y.cmp(&((space.y - 1) / 2)),
        ) {
            (Ordering::Less, Ordering::Less) => res[0] += 1,
            (Ordering::Greater, Ordering::Less) => res[1] += 1,
            (Ordering::Less, Ordering::Greater) => res[2] += 1,
            (Ordering::Greater, Ordering::Greater) => res[3] += 1,
            _ => (),
        }
    }
    res
}

fn part1_(space: Vector, input: &str) -> usize {
    let mut robots = parse(input);
    (0..100).for_each(|_| tick(space, &mut robots));
    quadrants(space, &robots).iter().product()
}

fn to_grid(space: Vector, robots: &[Robot]) -> Grid<bool> {
    let mut grid = Grid::new(false, space);
    for robot in robots {
        grid[robot.p] = true;
    }
    grid
}

fn contains_tree(space: Vector, robots: &[Robot]) -> bool {
    let grid = to_grid(space, robots);
    grid.keys().any(|v| {
        TREE.iter()
            .all(|(u, a)| grid.get(v + u).is_some_and(|b| a == b))
    })
}

pub fn part1(input: &str) -> usize {
    part1_(Vector::new(101, 103), input)
}

pub fn part2(input: &str) -> usize {
    let space = Vector::new(101, 103);
    let mut robots = parse(input);
    let mut seconds = 0;
    while !contains_tree(space, &robots) {
        tick(space, &mut robots);
        seconds += 1;
    }
    seconds
}

pub fn tests() {
    let example = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    ";
    assert_eq!(part1_(Vector::new(11, 7), example), 12);
}
