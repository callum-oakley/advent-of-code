use std::collections::HashSet;

use crate::{
    grid::{Adjacent, E, Grid, N, NE, NW, S, SE, SW, Vector, W},
    search,
};

fn parse(input: &str) -> (Vector, Grid<Vec<Vector>>) {
    let mut start = None;
    let mut pipes = Grid::parse(input, |pos, c| match c {
        '|' => vec![pos + N, pos + S],
        '-' => vec![pos + E, pos + W],
        'L' => vec![pos + N, pos + E],
        'J' => vec![pos + N, pos + W],
        '7' => vec![pos + S, pos + W],
        'F' => vec![pos + S, pos + E],
        '.' => vec![],
        'S' => {
            start = Some(pos);
            vec![]
        }
        _ => unreachable!(),
    });
    let start = start.unwrap();
    pipes[start] = start
        .adjacent4()
        .filter(|&v| pipes.get(v).is_some_and(|adj| adj.contains(&start)))
        .collect();
    assert_eq!(pipes[start].len(), 2);
    (start, pipes)
}

fn boundary(start: Vector, pipes: &Grid<Vec<Vector>>) -> Vec<Vector> {
    let mut res = vec![start];
    let mut seen = HashSet::from([start]);
    while let Some(&pos) = pipes[*res.last().unwrap()]
        .iter()
        .find(|&p| !seen.contains(p))
    {
        res.push(pos);
        seen.insert(pos);
    }
    res
}

fn area(start: Vector, boundary: &HashSet<Vector>) -> impl Iterator<Item = Vector> + '_ {
    search::breadth_first(
        start,
        |&state, push| {
            state
                .adjacent4()
                .filter(|a| !boundary.contains(a))
                .for_each(push);
        },
        search::id_filter(),
    )
}

fn count_area(start: Vector, boundary: &HashSet<Vector>, size: Vector) -> Option<usize> {
    let mut count = 0;
    for pos in area(start, boundary) {
        if pos.x == 0 || pos.y == 0 || pos.x == size.x - 1 || pos.y == size.y - 1 {
            // This is the outside area not the inside
            return None;
        }
        // Only count even points since they correspond to points before expansion
        if pos.x % 2 == 0 && pos.y % 2 == 0 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part1(input: &str) -> usize {
    let (start, pipes) = parse(input);
    boundary(start, &pipes).len() / 2
}

pub fn part2(input: &str) -> usize {
    let (start, pipes) = parse(input);
    let boundary = boundary(start, &pipes);

    // Expand the boundary so that there are empty spaces "between the pipes".
    let boundary = boundary
        .windows(2)
        .flat_map(|pair| vec![pair[0] * 2, pair[0] + pair[1]])
        .chain(vec![
            boundary[boundary.len() - 1] * 2,
            boundary[boundary.len() - 1] + boundary[0],
        ])
        .collect::<HashSet<_>>();
    let start = start * 2;
    let size = pipes.size * 2;

    for start in [start + NW, start + NE, start + SE, start + SW] {
        if let Some(count) = count_area(start, &boundary, size) {
            return count;
        }
    }
    unreachable!()
}

pub fn tests() {
    assert_eq!(part1(".....\n.S-7.\n.|.|.\n.L-J.\n....."), 4);
    assert_eq!(part1("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ..."), 8);

    let small_example = [
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ]
    .join("\n");
    let large_example = [
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ]
    .join("\n");
    let junk_example = [
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ]
    .join("\n");
    assert_eq!(part2(&small_example), 4);
    assert_eq!(part2(&large_example), 8);
    assert_eq!(part2(&junk_example), 10);
}
