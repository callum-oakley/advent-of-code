use num::Integer;

use crate::grid::Vector3;

#[derive(Clone)]
struct Moon {
    pos: Vector3,
    vel: Vector3,
}

fn parse(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| Moon {
            pos: crate::cast::str_to_vector3(line),
            vel: Vector3::zeros(),
        })
        .collect()
}

fn tick(system: &mut [Moon]) {
    for i in 0..system.len() {
        for j in 0..system.len() {
            for axis in 0..3 {
                match system[i].pos[axis].cmp(&system[j].pos[axis]) {
                    std::cmp::Ordering::Less => {
                        system[i].vel[axis] += 1;
                    }
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => {
                        system[i].vel[axis] -= 1;
                    }
                }
            }
        }
    }
    for moon in system {
        moon.pos += moon.vel;
    }
}

fn part1_(steps: usize, input: &str) -> i64 {
    let mut system = parse(input);
    for _ in 0..steps {
        tick(&mut system);
    }
    system
        .iter()
        .map(|moon| moon.pos.abs().sum() * moon.vel.abs().sum())
        .sum()
}

fn period(axis: usize, mut system: Vec<Moon>) -> usize {
    fn fingerprint(axis: usize, system: &[Moon]) -> Vec<(i64, i64)> {
        system
            .iter()
            .map(|moon| (moon.pos[axis], moon.vel[axis]))
            .collect()
    }

    let initial = fingerprint(axis, &system);

    for step in 1.. {
        tick(&mut system);
        if fingerprint(axis, &system) == initial {
            return step;
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> i64 {
    part1_(1000, input)
}

pub fn part2(input: &str) -> usize {
    let system = parse(input);
    // The movement in each axis is independent so we can find the period of each axis
    // independently and take the LCM.
    (0..3)
        .map(|axis| period(axis, system.clone()))
        .reduce(|a, b| Integer::lcm(&a, &b))
        .unwrap()
}

pub fn tests() {
    let example1 = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
    let example2 = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
    assert_eq!(part1_(10, example1), 179);
    assert_eq!(part1_(100, example2), 1940);
    assert_eq!(part2(example1), 2772);
    assert_eq!(part2(example2), 4_686_774_924);
}
