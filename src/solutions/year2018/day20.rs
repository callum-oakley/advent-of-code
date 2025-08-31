use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Vector, Z},
    search,
};

// Construct a map of the base as a graph by stepping through the regex keeping track of all the
// possible positions we could be in while matching the regex up to this point
fn expand(input: &str) -> HashMap<Vector, HashSet<Vector>> {
    let mut graph: HashMap<_, HashSet<_>> = HashMap::new();
    let mut active = HashSet::from([Z]);
    let mut forks = Vec::new();
    let mut alts: Vec<Vec<Vector>> = Vec::new();
    for c in input.chars() {
        match c {
            '^' | '$' => (),
            '(' => {
                // Save starting active positions so we can load them when we reach a '|'
                forks.push(active.clone());
                // To keep track of the different ending active positions we can reach in this group
                alts.push(Vec::new());
            }
            '|' => {
                // Amend the possible ending active positions for this group
                alts.last_mut().unwrap().extend(&active);
                // Reset active back to what it was at the last '('
                active.clone_from(forks.last().unwrap());
            }
            ')' => {
                // Add all alternative positions we could have reached by the end of this group
                active.extend(alts.pop().unwrap());
                forks.pop().unwrap();
            }
            _ => {
                let dir = crate::cast::char_to_vector(c);
                // Insert all the doors that must exist to move in this direction from the active
                // positions
                for &pos in &active {
                    graph.entry(pos).or_default().insert(pos + dir);
                    graph.entry(pos + dir).or_default().insert(pos);
                }
                // Move all the active positions in this direction
                active = active.into_iter().map(|pos| pos + dir).collect();
            }
        }
    }
    graph
}

#[derive(Clone)]
struct State {
    pos: Vector,
    dist: usize,
}

fn search(input: &str) -> impl Iterator<Item = State> {
    let graph = expand(input);
    search::breadth_first(
        State { pos: Z, dist: 0 },
        move |state, push| {
            for &pos in &graph[&state.pos] {
                push(State {
                    pos,
                    dist: state.dist + 1,
                });
            }
        },
        search::hash_filter(|state: &State| state.pos),
    )
}

pub fn part1(input: &str) -> usize {
    search(input).last().unwrap().dist
}

pub fn part2(input: &str) -> usize {
    search(input).filter(|s| s.dist >= 1000).count()
}

pub fn tests() {
    assert_eq!(part1("^WNE$"), 3);
    assert_eq!(part1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
    assert_eq!(part1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
    assert_eq!(
        part1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
        23,
    );
    assert_eq!(
        part1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
        31,
    );
}
