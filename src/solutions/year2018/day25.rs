use std::collections::HashSet;

use crate::grid::Vector4;

pub fn part1(input: &str) -> usize {
    let mut constellations: Vec<HashSet<Vector4>> = Vec::new();
    for point in input.lines().map(crate::cast::string_to_vector4) {
        let mut connected = HashSet::from([point]);
        let mut i = 0;
        while i < constellations.len() {
            if constellations[i]
                .iter()
                .any(|&p| (p - point).abs().sum() <= 3)
            {
                connected.extend(constellations.swap_remove(i));
                continue;
            }
            i += 1;
        }
        constellations.push(connected);
    }
    constellations.len()
}

pub fn tests() {
    assert_eq!(
        part1(
            "0,0,0,0
             3,0,0,0
             0,3,0,0
             0,0,3,0
             0,0,0,3
             0,0,0,6
             9,0,0,0
             12,0,0,0"
        ),
        2,
    );
    assert_eq!(
        part1(
            "-1,2,2,0
             0,0,2,-2
             0,0,0,-2
             -1,2,0,0
             -2,-2,-2,2
             3,0,2,-1
             -1,3,2,2
             -1,0,-1,0
             0,2,1,-2
             3,0,0,0"
        ),
        4,
    );
    assert_eq!(
        part1(
            "1,-1,0,1
             2,0,-1,0
             3,2,-1,0
             0,0,3,1
             0,0,-1,-1
             2,3,-2,0
             -2,2,0,0
             2,-2,0,-1
             1,-1,0,-1
             3,2,0,2"
        ),
        3,
    );
    assert_eq!(
        part1(
            "1,-1,-1,-2
             -2,-2,0,1
             0,2,1,3
             -2,3,-2,1
             0,2,3,-2
             -1,-1,1,-2
             0,-2,-1,0
             -2,2,3,-1
             1,2,2,0
             -1,-2,0,-2"
        ),
        8,
    );
}
