use std::collections::HashSet;

use crate::grid::{E, S, SE, Vector};

fn parse(input: &str) -> (HashSet<Vector>, HashSet<Vector>, Vector) {
    let mut east = HashSet::new();
    let mut south = HashSet::new();
    let mut size = Vector::new(0, 0);
    for (pos, c) in crate::grid::scan(input) {
        match c {
            '>' => {
                east.insert(pos);
            }
            'v' => {
                south.insert(pos);
            }
            _ => {}
        }
        size = pos + SE;
    }
    (east, south, size)
}

pub fn part1(input: &str) -> usize {
    let (mut east, mut south, size) = parse(input);
    let mut steps = 0;
    loop {
        steps += 1;
        let next_east: HashSet<Vector> = east
            .iter()
            .map(|&v| {
                let next_v = (v + E).zip_map(&size, |a, b| a.rem_euclid(b));
                if east.contains(&next_v) || south.contains(&next_v) {
                    v
                } else {
                    next_v
                }
            })
            .collect();
        let next_south: HashSet<Vector> = south
            .iter()
            .map(|&v| {
                let next_v = (v + S).zip_map(&size, |a, b| a.rem_euclid(b));
                if next_east.contains(&next_v) || south.contains(&next_v) {
                    v
                } else {
                    next_v
                }
            })
            .collect();
        if next_east == east && next_south == south {
            break;
        }
        east = next_east;
        south = next_south;
    }
    steps
}

pub fn tests() {
    let example = concat!(
        "v...>>.vv>\n",
        ".vv>>.vv..\n",
        ">>.>v>...v\n",
        ">>v>>.>.v.\n",
        "v>v.vv.v..\n",
        ">.>>..v...\n",
        ".vv..>.>v.\n",
        "v.v..>>v.v\n",
        "....v..v.>\n",
    );
    assert_eq!(part1(example), 58);
}
