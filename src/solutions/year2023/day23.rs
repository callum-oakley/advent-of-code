use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Adjacent, Grid, Vector},
    search,
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Vector),
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::parse(input, |_, c| match c {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' | '<' | '>' | 'v' => Tile::Slope(crate::cast::char_to_vector(c)),
        _ => unreachable!(),
    })
}

fn reachable(map: &Grid<Tile>, nodes: &HashSet<Vector>, start: Vector) -> HashMap<Vector, usize> {
    search::breadth_first(
        (start, 0),
        |&(pos, steps), push| {
            if nodes.contains(&pos) && pos != start {
                // stop
            } else if let Tile::Slope(dir) = map[pos] {
                push((pos + dir, steps + 1));
            } else {
                pos.adjacent4()
                    .filter(|&v| map.get(v).is_some_and(|&tile| tile != Tile::Forest))
                    .for_each(|v| push((v, steps + 1)));
            }
        },
        search::hash_filter(|&(pos, _)| pos),
    )
    .filter(|&(pos, _)| nodes.contains(&pos) && pos != start)
    .collect()
}

fn graph(map: &Grid<Tile>, start: Vector, end: Vector) -> HashMap<Vector, HashMap<Vector, usize>> {
    let mut nodes = HashSet::from([start, end]);
    nodes.extend(map.keys().filter(|&pos| {
        pos.adjacent4()
            .filter(|&v| map.get(v).is_some_and(|&tile| tile != Tile::Forest))
            .count()
            >= 3
    }));
    nodes
        .iter()
        .map(|&node| (node, reachable(map, &nodes, node)))
        .collect()
}

fn steps(graph: &HashMap<Vector, HashMap<Vector, usize>>, path: &[Vector]) -> usize {
    path.windows(2).map(|hop| graph[&hop[0]][&hop[1]]).sum()
}

fn part_(map: &Grid<Tile>) -> usize {
    let start = Vector::new(1, 0);
    let end = map.size - Vector::new(2, 1);
    let graph = graph(map, start, end);
    search::breadth_first(
        vec![start],
        |path, push| {
            let pos = *path.last().unwrap();
            if pos != end {
                for &p in graph[&pos].keys() {
                    if !path.contains(&p) {
                        let mut path = path.clone();
                        path.push(p);
                        push(path);
                    }
                }
            }
        },
        search::no_filter,
    )
    .filter(|path| *path.last().unwrap() == end)
    .map(|path| steps(&graph, &path))
    .max()
    .unwrap()
}

pub fn part1(input: &str) -> usize {
    part_(&parse(input))
}

pub fn part2(input: &str) -> usize {
    let mut map = parse(input);
    for pos in map.keys() {
        if let Tile::Slope(_) = map[pos] {
            map[pos] = Tile::Path;
        }
    }
    part_(&map)
}

pub fn tests() {
    let example = [
        "#.#####################",
        "#.......#########...###",
        "#######.#########.#.###",
        "###.....#.>.>.###.#.###",
        "###v#####.#v#.###.#.###",
        "###.>...#.#.#.....#...#",
        "###v###.#.#.#########.#",
        "###...#.#.#.......#...#",
        "#####.#.#.#######.#.###",
        "#.....#.#.#.......#...#",
        "#.#####.#.#.#########v#",
        "#.#...#...#...###...>.#",
        "#.#.#v#######v###.###v#",
        "#...#.>.#...>.>.#.###.#",
        "#####v#.#.###v#.#.###.#",
        "#.....#...#...#.#.#...#",
        "#.#########.###.#.#.###",
        "#...###...#...#...#.###",
        "###.###.#.###v#####v###",
        "#...#...#.#.>.>.#.>.###",
        "#.###.###.#.###.#.#v###",
        "#.....###...###...#...#",
        "#####################.#",
    ]
    .join("\n");
    assert_eq!(part1(&example), 94);
    assert_eq!(part2(&example), 154);
}
