use crate::{
    grid::{E, N, Z},
    part::Part,
};

fn part_(part: Part, input: &str) -> i64 {
    let mut pos = Z;
    let mut dir = match part {
        Part::One => E,
        Part::Two => 10 * E + N,
    };
    for line in input.lines() {
        let c = line.chars().next().unwrap();
        let n: i64 = line[1..].parse().unwrap();
        match c {
            'N' | 'E' | 'S' | 'W' => match part {
                Part::One => pos += n * crate::cast::char_to_vector(c),
                Part::Two => dir += n * crate::cast::char_to_vector(c),
            },
            'L' | 'R' => {
                dir = crate::cast::char_to_turn(c).pow(u32::try_from(n).unwrap() / 90) * dir;
            }
            'F' => pos += n * dir,
            _ => unreachable!(),
        }
    }
    pos.abs().sum()
}

pub fn part1(input: &str) -> i64 {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> i64 {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "F10\nN3\nF7\nR90\nF11";
    assert_eq!(part1(example), 25);
    assert_eq!(part2(example), 286);
}
