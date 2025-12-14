use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Adjacent, E, Grid, N, S, Vector, W},
    search,
};

struct Maze {
    passages: HashSet<Vector>,
    outer_portals: HashMap<Vector, Vector>,
    inner_portals: HashMap<Vector, Vector>,
    start: Vector,
    finish: Vector,
}

fn parse(input: &str) -> Maze {
    let g = Grid::parse(input, |_, c| c);

    let mut passages = HashSet::new();
    let mut outer_portals: HashMap<(char, char), Vector> = HashMap::new();
    let mut inner_portals: HashMap<(char, char), Vector> = HashMap::new();
    let mut start = None;
    let mut finish = None;

    for (pos, &c) in &g {
        if c == '.' {
            passages.insert(pos);
            for dir in [N, E, S, W] {
                if let (Some(&d), Some(&e)) = (g.get(pos + dir), g.get(pos + dir * 2))
                    && d.is_ascii_uppercase()
                    && e.is_ascii_uppercase()
                {
                    let label = if dir == N || dir == W { (e, d) } else { (d, e) };
                    match label {
                        ('A', 'A') => {
                            start = Some(pos);
                        }
                        ('Z', 'Z') => {
                            finish = Some(pos);
                        }
                        _ => {
                            if pos.y == 2
                                || pos.y == g.size.y - 3
                                || pos.x == 2
                                || pos.x == g.size.x - 3
                            {
                                outer_portals.insert(label, pos);
                            } else {
                                inner_portals.insert(label, pos);
                            }
                        }
                    }
                }
            }
        }
    }

    Maze {
        passages,
        outer_portals: outer_portals
            .iter()
            .map(|(&label, &pos)| (pos, inner_portals[&label]))
            .collect(),
        inner_portals: inner_portals
            .iter()
            .map(|(&label, &pos)| (pos, outer_portals[&label]))
            .collect(),
        start: start.unwrap(),
        finish: finish.unwrap(),
    }
}

pub fn part1(input: &str) -> u32 {
    struct State {
        pos: Vector,
        steps: u32,
    }

    let maze = parse(input);
    search::breadth_first(
        State {
            pos: maze.start,
            steps: 0,
        },
        move |state, push| {
            for p in state.pos.adjacent4() {
                if maze.passages.contains(&p) {
                    push(State {
                        pos: p,
                        steps: state.steps + 1,
                    });
                }
            }

            if let Some(&p) = maze
                .outer_portals
                .get(&state.pos)
                .or(maze.inner_portals.get(&state.pos))
            {
                push(State {
                    pos: p,
                    steps: state.steps + 1,
                });
            }
        },
        search::hash_filter(|state: &State| state.pos),
    )
    .find(|state| state.pos == maze.finish)
    .unwrap()
    .steps
}

pub fn part2(input: &str) -> u32 {
    struct State {
        pos: Vector,
        level: u32,
        steps: u32,
    }

    let maze = parse(input);
    search::breadth_first(
        State {
            pos: maze.start,
            level: 0,
            steps: 0,
        },
        move |state, push| {
            for p in state.pos.adjacent4() {
                if maze.passages.contains(&p) {
                    push(State {
                        pos: p,
                        level: state.level,
                        steps: state.steps + 1,
                    });
                }
            }

            if let Some(&p) = maze.outer_portals.get(&state.pos) {
                if state.level > 0 {
                    push(State {
                        pos: p,
                        level: state.level - 1,
                        steps: state.steps + 1,
                    });
                }
            } else if let Some(&p) = maze.inner_portals.get(&state.pos) {
                push(State {
                    pos: p,
                    level: state.level + 1,
                    steps: state.steps + 1,
                });
            }
        },
        search::hash_filter(|state: &State| (state.pos, state.level)),
    )
    .find(|state| state.pos == maze.finish && state.level == 0)
    .unwrap()
    .steps
}

#[expect(clippy::too_many_lines)]
pub fn tests() {
    let example0 = [
        "         A           ",
        "         A           ",
        "  #######.#########  ",
        "  #######.........#  ",
        "  #######.#######.#  ",
        "  #######.#######.#  ",
        "  #######.#######.#  ",
        "  #####  B    ###.#  ",
        "BC...##  C    ###.#  ",
        "  ##.##       ###.#  ",
        "  ##...DE  F  ###.#  ",
        "  #####    G  ###.#  ",
        "  #########.#####.#  ",
        "DE..#######...###.#  ",
        "  #.#########.###.#  ",
        "FG..#########.....#  ",
        "  ###########.#####  ",
        "             Z       ",
        "             Z       ",
    ]
    .join("\n");
    let example1 = [
        "                   A               ",
        "                   A               ",
        "  #################.#############  ",
        "  #.#...#...................#.#.#  ",
        "  #.#.#.###.###.###.#########.#.#  ",
        "  #.#.#.......#...#.....#.#.#...#  ",
        "  #.#########.###.#####.#.#.###.#  ",
        "  #.............#.#.....#.......#  ",
        "  ###.###########.###.#####.#.#.#  ",
        "  #.....#        A   C    #.#.#.#  ",
        "  #######        S   P    #####.#  ",
        "  #.#...#                 #......VT",
        "  #.#.#.#                 #.#####  ",
        "  #...#.#               YN....#.#  ",
        "  #.###.#                 #####.#  ",
        "DI....#.#                 #.....#  ",
        "  #####.#                 #.###.#  ",
        "ZZ......#               QG....#..AS",
        "  ###.###                 #######  ",
        "JO..#.#.#                 #.....#  ",
        "  #.#.#.#                 ###.#.#  ",
        "  #...#..DI             BU....#..LF",
        "  #####.#                 #.#####  ",
        "YN......#               VT..#....QG",
        "  #.###.#                 #.###.#  ",
        "  #.#...#                 #.....#  ",
        "  ###.###    J L     J    #.#.###  ",
        "  #.....#    O F     P    #.#...#  ",
        "  #.###.#####.#.#####.#####.###.#  ",
        "  #...#.#.#...#.....#.....#.#...#  ",
        "  #.#####.###.###.#.#.#########.#  ",
        "  #...#.#.....#...#.#.#.#.....#.#  ",
        "  #.###.#####.###.###.#.#.#######  ",
        "  #.#.........#...#.............#  ",
        "  #########.###.###.#############  ",
        "           B   J   C               ",
        "           U   P   P               ",
    ]
    .join("\n");
    let example2 = [
        "             Z L X W       C                 ",
        "             Z P Q B       K                 ",
        "  ###########.#.#.#.#######.###############  ",
        "  #...#.......#.#.......#.#.......#.#.#...#  ",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
        "  #.###.#######.###.###.#.###.###.#.#######  ",
        "  #...#.......#.#...#...#.............#...#  ",
        "  #.#########.#######.#.#######.#######.###  ",
        "  #...#.#    F       R I       Z    #.#.#.#  ",
        "  #.###.#    D       E C       H    #.#.#.#  ",
        "  #.#...#                           #...#.#  ",
        "  #.###.#                           #.###.#  ",
        "  #.#....OA                       WB..#.#..ZH",
        "  #.###.#                           #.#.#.#  ",
        "CJ......#                           #.....#  ",
        "  #######                           #######  ",
        "  #.#....CK                         #......IC",
        "  #.###.#                           #.###.#  ",
        "  #.....#                           #...#.#  ",
        "  ###.###                           #.#.#.#  ",
        "XF....#.#                         RF..#.#.#  ",
        "  #####.#                           #######  ",
        "  #......CJ                       NM..#...#  ",
        "  ###.#.#                           #.###.#  ",
        "RE....#.#                           #......RF",
        "  ###.###        X   X       L      #.#.#.#  ",
        "  #.....#        F   Q       P      #.#.#.#  ",
        "  ###.###########.###.#######.#########.###  ",
        "  #.....#...#.....#.......#...#.....#.#...#  ",
        "  #####.#.###.#######.#######.###.###.#.#.#  ",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
        "  #.......#.....#.#...#...............#...#  ",
        "  #############.#.#.###.###################  ",
        "               A O F   N                     ",
        "               A A D   M                     ",
    ]
    .join("\n");

    assert_eq!(part1(&example0), 23);
    assert_eq!(part1(&example1), 58);

    assert_eq!(part2(&example2), 396);
}
