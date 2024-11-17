use std::sync::LazyLock;

use regex::Regex;

use crate::{
    grid::{self, Grid, Vector, E, S},
    ocr,
};

enum Instruction {
    Rect(Vector),
    RotRow { y: i32, by: i32 },
    RotCol { x: i32, by: i32 },
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
    input.lines().map(|line| {
        let line = line.trim();
        let mut nums = RE.find_iter(line);
        if line.starts_with("rotate row") {
            Instruction::RotRow {
                y: nums.next().unwrap().as_str().parse().unwrap(),
                by: nums.next().unwrap().as_str().parse().unwrap(),
            }
        } else if line.starts_with("rotate col") {
            Instruction::RotCol {
                x: nums.next().unwrap().as_str().parse().unwrap(),
                by: nums.next().unwrap().as_str().parse().unwrap(),
            }
        } else {
            Instruction::Rect(Vector::new(
                nums.next().unwrap().as_str().parse().unwrap(),
                nums.next().unwrap().as_str().parse().unwrap(),
            ))
        }
    })
}

fn reverse(screen: &mut Grid<bool>, dir: Vector, mut start: Vector, mut end: Vector) {
    end -= dir;
    while grid::reading_ord_key(start) < grid::reading_ord_key(end) {
        let tmp = screen[start];
        screen[start] = screen[end];
        screen[end] = tmp;

        start += dir;
        end -= dir;
    }
}

fn block_swap(screen: &mut Grid<bool>, dir: Vector, start: Vector, mid: Vector, end: Vector) {
    reverse(screen, dir, start, mid);
    reverse(screen, dir, mid, end);
    reverse(screen, dir, start, end);
}

fn part_(size: Vector, input: &str) -> Grid<bool> {
    let mut screen = Grid::new(false, size);
    for instruction in parse(input) {
        match instruction {
            Instruction::Rect(p) => {
                for x in 0..p.x {
                    for y in 0..p.y {
                        screen[[x, y]] = true;
                    }
                }
            }
            Instruction::RotRow { y, by } => block_swap(
                &mut screen,
                E,
                Vector::new(0, y),
                Vector::new(size.x - by, y),
                Vector::new(size.x, y),
            ),
            Instruction::RotCol { x, by } => block_swap(
                &mut screen,
                S,
                Vector::new(x, 0),
                Vector::new(x, size.y - by),
                Vector::new(x, size.y),
            ),
        }
    }
    screen
}

pub fn part1(input: &str) -> usize {
    part_(Vector::new(50, 6), input)
        .into_values()
        .filter(|p| *p)
        .count()
}

pub fn part2(input: &str) -> &str {
    ocr::parse(&part_(Vector::new(50, 6), input).to_string())
}

pub fn tests() {
    let example = "rect 3x2
                   rotate column x=1 by 1
                   rotate row y=0 by 4
                   rotate column x=1 by 1";
    assert_eq!(
        part_(Vector::new(7, 3), example).to_string(),
        ".#..#.#\n#.#....\n.#.....\n",
    );
}
