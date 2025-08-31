use std::collections::HashSet;

use crate::{
    grid::{self, Adjacent, Vector},
    search,
};

// Represent a crop square by a pair of position + crop, and the whole garden by the set of crop
// squares.
fn parse(input: &str) -> HashSet<(Vector, char)> {
    grid::scan(input).collect()
}

// Represent the edge of a square by a pair indicating the position of the square and the compass
// direction of the edge (N, E, S, or W). Then the perimeter is a set of edges.
fn perimeter(region: &HashSet<(Vector, char)>) -> HashSet<(Vector, char)> {
    region
        .iter()
        .flat_map(|&(v, crop)| {
            "NESW".chars().map(move |c| (v, c)).filter(move |&(u, c)| {
                !region.contains(&(u + crate::cast::char_to_vector(c), crop))
            })
        })
        .collect()
}

// Break a garden into subsets such that for each subset:
//
// - The positions form a single connected region, and
// - The chars (either a crop or a direction) are all the same.
//
// This works both for finding the initial breakdown of crop regions, and for finding the breakdown
// of a perimeter into edges!
fn regions(garden: &HashSet<(Vector, char)>) -> Vec<HashSet<(Vector, char)>> {
    let mut res: Vec<HashSet<(Vector, char)>> = Vec::new();
    for &(v, c) in garden {
        if res.iter().any(|r| r.contains(&(v, c))) {
            continue;
        }
        res.push(
            search::breadth_first(
                v,
                |&v, push| {
                    v.adjacent4()
                        .filter(|&u| garden.contains(&(u, c)))
                        .for_each(push);
                },
                search::id_filter(),
            )
            .map(|v| (v, c))
            .collect(),
        );
    }
    res
}

pub fn part1(input: &str) -> usize {
    regions(&parse(input))
        .iter()
        .map(|region| region.len() * perimeter(region).len())
        .sum()
}

pub fn part2(input: &str) -> usize {
    regions(&parse(input))
        .iter()
        .map(|region| region.len() * regions(&perimeter(region)).len())
        .sum()
}

pub fn tests() {
    let example0 = ["AAAA", "BBCD", "BBCC", "EEEC"].join("\n");
    let example1 = ["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"].join("\n");
    let example2 = [
        "RRRRIICCFF",
        "RRRRIICCCF",
        "VVRRRCCFFF",
        "VVRCCCJFFF",
        "VVVVCJJCFE",
        "VVIVCCJJEE",
        "VVIIICJJEE",
        "MIIIIIJJEE",
        "MIIISIJEEE",
        "MMMISSJEEE",
    ]
    .join("\n");
    let example3 = ["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"].join("\n");
    let example4 = ["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"].join("\n");

    assert_eq!(part1(&example0), 140);
    assert_eq!(part1(&example1), 772);
    assert_eq!(part1(&example2), 1930);

    assert_eq!(part2(&example0), 80);
    assert_eq!(part2(&example1), 436);
    assert_eq!(part2(&example2), 1206);
    assert_eq!(part2(&example3), 236);
    assert_eq!(part2(&example4), 368);
}
