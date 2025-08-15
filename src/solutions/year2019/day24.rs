use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::grid::{Adjacent, Grid, Vector};

fn biodiversity(bugs: &Grid<bool>) -> u32 {
    bugs.values()
        .enumerate()
        .map(|(i, &bug)| if bug { 1 << i } else { 0 })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    let mut bugs = Grid::parse(input, |_, c| c == '#');
    let mut seen = HashSet::new();

    while !seen.contains(&bugs) {
        seen.insert(bugs.clone());
        let mut bugs_next = bugs.clone();
        for (pos, bug) in &bugs {
            let adjacent = pos
                .adjacent4()
                .filter(|&v| bugs.get(v).is_some_and(|&bug| bug))
                .count();
            bugs_next[pos] = adjacent == 1 || !bug && adjacent == 2;
        }
        bugs = bugs_next;
    }

    biodiversity(&bugs)
}

fn adjacent(bugs: &HashMap<i64, Grid<bool>>, depth: i64, pos: Vector) -> usize {
    let mut res = 0;
    for p in pos.adjacent4() {
        res += if p.x < 0 {
            usize::from(bugs[&(depth - 1)][[1, 2]])
        } else if p.y < 0 {
            usize::from(bugs[&(depth - 1)][[2, 1]])
        } else if p.x > 4 {
            usize::from(bugs[&(depth - 1)][[3, 2]])
        } else if p.y > 4 {
            usize::from(bugs[&(depth - 1)][[2, 3]])
        } else if p == Vector::new(2, 2) {
            match Into::<[i64; 2]>::into(pos) {
                [2, 1] => (0..5).filter(|&x| bugs[&(depth + 1)][[x, 0]]).count(),
                [1, 2] => (0..5).filter(|&y| bugs[&(depth + 1)][[0, y]]).count(),
                [3, 2] => (0..5).filter(|&y| bugs[&(depth + 1)][[4, y]]).count(),
                [2, 3] => (0..5).filter(|&x| bugs[&(depth + 1)][[x, 4]]).count(),
                _ => unreachable!(),
            }
        } else {
            usize::from(bugs[&depth][p])
        }
    }
    res
}

fn part2_(minutes: usize, input: &str) -> usize {
    let mut bugs = HashMap::from([
        (-2, Grid::new(false, [5, 5])),
        (-1, Grid::new(false, [5, 5])),
        (0, Grid::parse(input, |_, c| c == '#')),
        (1, Grid::new(false, [5, 5])),
        (2, Grid::new(false, [5, 5])),
    ]);
    let mut bugs_next = bugs.clone();

    let mut min_depth = -1;
    let mut max_depth = 1;

    for _ in 0..minutes {
        for depth in min_depth..=max_depth {
            for (pos, bug) in &bugs[&depth] {
                if pos == Vector::new(2, 2) {
                    continue;
                }
                let adjacent = adjacent(&bugs, depth, pos);
                bugs_next.get_mut(&depth).unwrap()[pos] = adjacent == 1 || !bug && adjacent == 2;
            }
        }

        mem::swap(&mut bugs, &mut bugs_next);

        if bugs[&min_depth].values().any(|&bug| bug) {
            min_depth -= 1;
            bugs.insert(min_depth - 1, Grid::new(false, [5, 5]));
            bugs_next.insert(min_depth - 1, Grid::new(false, [5, 5]));
        }

        if bugs[&max_depth].values().any(|&bug| bug) {
            max_depth += 1;
            bugs.insert(max_depth + 1, Grid::new(false, [5, 5]));
            bugs_next.insert(max_depth + 1, Grid::new(false, [5, 5]));
        }
    }

    bugs.values()
        .flat_map(Grid::values)
        .filter(|&&bug| bug)
        .count()
}

pub fn part2(input: &str) -> usize {
    part2_(200, input)
}

pub fn tests() {
    assert_eq!(part1("....#\n#..#.\n#..##\n..#..\n#...."), 2_129_920);
    assert_eq!(part2_(10, "....#\n#..#.\n#.?##\n..#..\n#...."), 99);
}
