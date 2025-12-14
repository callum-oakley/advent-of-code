use std::collections::HashMap;

use crate::{
    grid::{E, Grid, LEFT, RIGHT, Turn, Vector},
    search,
    uniq::Uniq,
};

#[derive(Clone)]
struct State {
    pos: Vector,
    dir: Vector,
    score: usize,
    path: Vec<Vector>,
}

impl State {
    fn step(&mut self) {
        self.pos += self.dir;
        self.score += 1;
        self.path.push(self.pos);
    }

    fn run(&self, maze: &Grid<char>) -> Self {
        let mut state = self.clone();
        state.step();
        while maze[state.pos + state.dir] != '#'
            && maze[state.pos + LEFT * state.dir] == '#'
            && maze[state.pos + RIGHT * state.dir] == '#'
        {
            state.step();
        }
        state
    }

    fn turn(&self, turn: Turn) -> Self {
        let mut state = self.clone();
        state.dir = turn * state.dir;
        state.score += 1000;
        state
    }
}

fn search(maze: &Grid<char>) -> impl Iterator<Item = State> + '_ {
    let start = maze.keys().find(|&v| maze[v] == 'S').unwrap();
    let mut lowest_score = HashMap::new();
    search::dijkstra(
        State {
            pos: start,
            dir: E,
            score: 0,
            path: vec![start],
        },
        |state, push| {
            if maze[state.pos] != 'E' {
                if maze[state.pos + state.dir] != '#' {
                    push(state.run(maze));
                }
                for turn in [LEFT, RIGHT] {
                    if maze[state.pos + turn * state.dir] != '#' {
                        push(state.turn(turn).run(maze));
                    }
                }
            }
        },
        // Track the lowest score that we see each (pos, dir) and only consider optimal branches.
        move |state| {
            if let Some(&score) = lowest_score.get(&(state.pos, state.dir)) {
                state.score == score
            } else {
                lowest_score.insert((state.pos, state.dir), state.score);
                true
            }
        },
        |state| state.score,
    )
}

pub fn part1(input: &str) -> usize {
    let maze = Grid::parse(input, |_, c| c);

    search(&maze)
        .find(|state| maze[state.pos] == 'E')
        .unwrap()
        .score
}

pub fn part2(input: &str) -> usize {
    let min_score = part1(input);
    let maze = Grid::parse(input, |_, c| c);
    search(&maze)
        .filter(|state| state.score == min_score && maze[state.pos] == 'E')
        .flat_map(|state| state.path)
        .uniq()
        .count()
}

pub fn tests() {
    let example0 = [
        "###############",
        "#.......#....E#",
        "#.#.###.#.###.#",
        "#.....#.#...#.#",
        "#.###.#####.#.#",
        "#.#.#.......#.#",
        "#.#.#####.###.#",
        "#...........#.#",
        "###.#.#####.#.#",
        "#...#.....#.#.#",
        "#.#.#.###.#.#.#",
        "#.....#...#.#.#",
        "#.###.#.#.#.#.#",
        "#S..#.....#...#",
        "###############",
    ]
    .join("\n");
    let example1 = [
        "#################",
        "#...#...#...#..E#",
        "#.#.#.#.#.#.#.#.#",
        "#.#.#.#...#...#.#",
        "#.#.#.#.###.#.#.#",
        "#...#.#.#.....#.#",
        "#.#.#.#.#.#####.#",
        "#.#...#.#.#.....#",
        "#.#.#####.#.###.#",
        "#.#.#.......#...#",
        "#.#.###.#####.###",
        "#.#.#...#.....#.#",
        "#.#.#.#####.###.#",
        "#.#.#.........#.#",
        "#.#.#.#########.#",
        "#S#.............#",
        "#################",
    ]
    .join("\n");
    assert_eq!(part1(&example0), 7036);
    assert_eq!(part1(&example1), 11048);
    assert_eq!(part2(&example0), 45);
    assert_eq!(part2(&example1), 64);
}
