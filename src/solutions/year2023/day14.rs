use std::collections::HashMap;

use crate::grid::{Point, Rect, E, N, S, W};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Ball,
    Block,
}

fn parse(input: &str) -> Rect<Tile> {
    Rect::parse(input, |_, c| match c {
        '.' => Tile::Empty,
        'O' => Tile::Ball,
        '#' => Tile::Block,
        _ => unreachable!(),
    })
}

fn roll(platform: &mut Rect<Tile>, dir: Point, mut ball: Point) {
    platform[ball] = Tile::Empty;
    while platform.get(ball + dir) == Some(&Tile::Empty) {
        ball += dir;
    }
    platform[ball] = Tile::Ball;
}

fn tilt(platform: &mut Rect<Tile>, dir: Point) {
    if dir == N || dir == W {
        for y in 0..platform.size.y {
            for x in 0..platform.size.x {
                if platform[Point { y, x }] == Tile::Ball {
                    roll(platform, dir, Point { y, x });
                }
            }
        }
    } else {
        for y in (0..platform.size.y).rev() {
            for x in (0..platform.size.x).rev() {
                if platform[Point { y, x }] == Tile::Ball {
                    roll(platform, dir, Point { y, x });
                }
            }
        }
    }
}

fn score(platform: &Rect<Tile>) -> i32 {
    platform
        .iter()
        .filter(|(_, &tile)| tile == Tile::Ball)
        .map(|(pos, _)| platform.size.y - pos.y)
        .sum()
}

pub fn part1(input: &str) -> i32 {
    let mut platform = parse(input);
    tilt(&mut platform, N);
    score(&platform)
}

pub fn part2(input: &str) -> i32 {
    fn spin(platform: &mut Rect<Tile>) {
        for dir in [N, W, S, E] {
            tilt(platform, dir);
        }
    }

    fn find_cycle(platform: &mut Rect<Tile>) -> (usize, usize) {
        let mut seen = HashMap::new();
        for spins in 0.. {
            if let Some(start) = seen.get(platform) {
                return (spins, spins - start);
            }
            seen.insert(platform.clone(), spins);
            spin(platform);
        }
        unreachable!()
    }

    let mut platform = parse(input);
    let (spins, period) = find_cycle(&mut platform);
    for _ in 0..(1_000_000_000 - spins) % period {
        spin(&mut platform);
    }
    score(&platform)
}

pub fn tests() {
    let example = [
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ]
    .join("\n");
    assert_eq!(part1(&example), 136);
    assert_eq!(part2(&example), 64);
}
