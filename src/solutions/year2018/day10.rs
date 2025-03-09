use regex::Regex;

use crate::grid::{Bounds, IntoVector, Vector};

struct Light {
    position: Vector,
    velocity: Vector,
}

fn parse(input: &str) -> Vec<Light> {
    Regex::new(r"position=<([^>]+)> velocity=<([^>]+)>")
        .unwrap()
        .captures_iter(input)
        .map(|captures| Light {
            position: captures[1].into_vector(),
            velocity: captures[2].into_vector(),
        })
        .collect()
}

fn tick(lights: &mut [Light]) {
    for light in lights {
        light.position += light.velocity;
    }
}

fn untick(lights: &mut [Light]) {
    for light in lights {
        light.position -= light.velocity;
    }
}

fn height(lights: &[Light]) -> i32 {
    let bounds = Bounds::new(lights.iter().map(|light| light.position));
    bounds.max.y - bounds.min.y + 1
}

fn part_(input: &str) -> (&str, usize) {
    let mut lights = parse(input);
    let mut h = height(&lights);
    for t in 0.. {
        tick(&mut lights);
        let next_h = height(&lights);
        if next_h < h {
            h = next_h;
        } else {
            untick(&mut lights);
            return (
                crate::ocr::parse(lights.iter().map(|light| light.position)),
                t,
            );
        }
    }
    unreachable!();
}

pub fn part1(input: &str) -> &str {
    let (res, _) = part_(input);
    res
}

pub fn part2(input: &str) -> usize {
    let (_, t) = part_(input);
    t
}

pub fn tests() {
    let example = "position=< 9,  1> velocity=< 0,  2>
                   position=< 7,  0> velocity=<-1,  0>
                   position=< 3, -2> velocity=<-1,  1>
                   position=< 6, 10> velocity=<-2, -1>
                   position=< 2, -4> velocity=< 2,  2>
                   position=<-6, 10> velocity=< 2, -2>
                   position=< 1,  8> velocity=< 1, -1>
                   position=< 1,  7> velocity=< 1,  0>
                   position=<-3, 11> velocity=< 1, -2>
                   position=< 7,  6> velocity=<-1, -1>
                   position=<-2,  3> velocity=< 1,  0>
                   position=<-4,  3> velocity=< 2,  0>
                   position=<10, -3> velocity=<-1,  1>
                   position=< 5, 11> velocity=< 1, -2>
                   position=< 4,  7> velocity=< 0, -1>
                   position=< 8, -2> velocity=< 0,  1>
                   position=<15,  0> velocity=<-2,  0>
                   position=< 1,  6> velocity=< 1,  0>
                   position=< 8,  9> velocity=< 0, -1>
                   position=< 3,  3> velocity=<-1,  1>
                   position=< 0,  5> velocity=< 0, -1>
                   position=<-2,  2> velocity=< 2,  0>
                   position=< 5, -2> velocity=< 1,  2>
                   position=< 1,  4> velocity=< 2,  1>
                   position=<-2,  7> velocity=< 2, -2>
                   position=< 3,  6> velocity=<-1, -1>
                   position=< 5,  0> velocity=< 1,  0>
                   position=<-6,  0> velocity=< 2,  0>
                   position=< 5,  9> velocity=< 1, -2>
                   position=<14,  7> velocity=<-2,  0>
                   position=<-3,  6> velocity=< 2, -1>";
    assert_eq!(part1(example), "HI");
    assert_eq!(part2(example), 3);
}
