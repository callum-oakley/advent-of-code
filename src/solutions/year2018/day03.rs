use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;

use crate::grid::{Grid, Vector};

struct Claim {
    id: u32,
    pos: Vector,
    size: Vector,
}

impl From<&str> for Claim {
    fn from(s: &str) -> Self {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"#(\d+) @ (\d+,\d+): (\d+x\d+)").unwrap());
        let captures = RE.captures(s).unwrap();
        Claim {
            id: captures[1].parse().unwrap(),
            pos: crate::cast::str_to_vector(&captures[2]),
            size: crate::cast::str_to_vector(&captures[3]),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    let mut fabric = Grid::new(0, [1000, 1000]);
    for claim in input.lines().map(Claim::from) {
        for x in claim.pos.x..claim.pos.x + claim.size.x {
            for y in claim.pos.y..claim.pos.y + claim.size.y {
                fabric[[x, y]] += 1;
                if fabric[[x, y]] == 2 {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn part2(input: &str) -> u32 {
    let mut disjoint = HashSet::new();
    let mut fabric = Grid::new(None, [1000, 1000]);
    for claim in input.lines().map(Claim::from) {
        disjoint.insert(claim.id);
        for x in claim.pos.x..claim.pos.x + claim.size.x {
            for y in claim.pos.y..claim.pos.y + claim.size.y {
                if let Some(id) = fabric[[x, y]] {
                    disjoint.remove(&id);
                    disjoint.remove(&claim.id);
                }
                fabric[[x, y]] = Some(claim.id);
            }
        }
    }
    assert_eq!(disjoint.len(), 1);
    disjoint.into_iter().next().unwrap()
}

pub fn tests() {
    let example = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    assert_eq!(part1(example), 4);
    assert_eq!(part2(example), 3);
}
