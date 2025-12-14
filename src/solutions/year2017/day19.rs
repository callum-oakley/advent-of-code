use crate::grid::{Grid, LEFT, RIGHT, S, Vector};

fn part_(input: &str) -> (String, usize) {
    let grid = Grid::parse(input, |_, c| c);

    let start_x = (0..grid.size.x).find(|&x| grid[[x, 0]] == '|').unwrap();
    let mut pos = Vector::new(start_x, 0);
    let mut dir = S;

    let mut res = (String::new(), 0);

    while grid[pos] != ' ' {
        if grid[pos].is_alphabetic() {
            res.0.push(grid[pos]);
        } else if grid[pos] == '+' {
            if grid[pos + LEFT * dir] != ' ' {
                dir = LEFT * dir;
            } else if grid[pos + RIGHT * dir] != ' ' {
                dir = RIGHT * dir;
            } else {
                unreachable!();
            }
        }
        pos += dir;
        res.1 += 1;
    }

    res
}

pub fn part1(input: &str) -> String {
    part_(input).0
}

pub fn part2(input: &str) -> usize {
    part_(input).1
}

pub fn tests() {
    let example = [
        "     |          ",
        "     |  +--+    ",
        "     A  |  C    ",
        " F---|----E|--+ ",
        "     |  |  |  D ",
        "     +B-+  +--+ ",
        "                ",
    ];
    assert_eq!(part1(&example.join("\n")), "ABCDEF");
    assert_eq!(part2(&example.join("\n")), 38);
}
