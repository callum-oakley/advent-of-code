use crate::grid::{self, Bounds, Vector};

fn parse(expansion: i64, input: &str) -> Vec<Vector> {
    let mut galaxies: Vec<_> = grid::scan(input)
        .filter(|&(_, c)| c == '#')
        .map(|(pos, _)| pos)
        .collect();

    let mut size = Bounds::new(galaxies.iter().copied()).size();
    for axis in [0, 1] {
        let mut a = 0;
        while a < size[axis] {
            if galaxies.iter().any(|galaxy| galaxy[axis] == a) {
                a += 1;
            } else {
                for galaxy in &mut galaxies {
                    if galaxy[axis] > a {
                        galaxy[axis] += expansion - 1;
                    }
                }
                size[axis] += expansion - 1;
                a += expansion;
            }
        }
    }

    galaxies
}

fn part_(expansion: i64, input: &str) -> i64 {
    let galaxies = parse(expansion, input);
    let mut res = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            res += (galaxies[j] - galaxies[i]).abs().sum();
        }
    }
    res
}

pub fn part1(input: &str) -> i64 {
    part_(2, input)
}

pub fn part2(input: &str) -> i64 {
    part_(1_000_000, input)
}

pub fn tests() {
    let example = [
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ]
    .join("\n");
    assert_eq!(part1(&example), 374);
    assert_eq!(part_(10, &example), 1030);
    assert_eq!(part_(100, &example), 8410);
}
