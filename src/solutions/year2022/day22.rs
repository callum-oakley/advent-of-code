use std::{collections::HashMap, sync::LazyLock};

use nalgebra::vector;
use regex::Regex;

use crate::grid::{E, N, S, SE, Turn, Vector, W};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Forward(usize),
    Turn(Turn),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Vector,
    dir: Vector,
}

impl State {
    fn new(pos: Vector, dir: Vector) -> Self {
        State { pos, dir }
    }
}

fn parse(input: &str) -> (HashMap<Vector, Tile>, impl Iterator<Item = Instruction>) {
    static INSTRUCTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+|L|R").unwrap());

    let (board, instructions) = input.split_once("\n\n").unwrap();

    let board = crate::grid::scan(board)
        .filter_map(|(v, c)| match c {
            '.' => Some((v, Tile::Empty)),
            '#' => Some((v, Tile::Wall)),
            _ => None,
        })
        .collect();

    let instructions = INSTRUCTION.find_iter(instructions).map(|m| {
        let m = m.as_str();
        match m {
            "L" | "R" => Instruction::Turn(crate::cast::str_to_turn(m)),
            _ => Instruction::Forward(m.parse().unwrap()),
        }
    });

    (board, instructions)
}

fn step(
    board: &HashMap<Vector, Tile>,
    portals: Option<&HashMap<State, State>>,
    mut state: State,
) -> Option<State> {
    if board.contains_key(&(state.pos + state.dir)) {
        state.pos += state.dir;
    } else if let Some(portals) = portals {
        state = portals[&state];
    } else {
        while board.contains_key(&(state.pos - state.dir)) {
            state.pos -= state.dir;
        }
    }

    match board[&state.pos] {
        Tile::Empty => Some(state),
        Tile::Wall => None,
    }
}

// Identify faces by their position on a grid as if each face was a single square:
//
//     +---+---+---+---+
//     |0,0|1,0|2,0|3,0|
//     +---+---+---+---+
//     |0,1|1,1|2,1|3,1|
//     +---+---+---+---+
//     |0,2|1,2|2,2|3,2|
//     +---+---+---+---+
//
// and then we can generate the full mapping from the seven joins between faces.
fn portals(side_len: i64, sides: &[(State, State)]) -> HashMap<State, State> {
    fn edge(side_len: i64, face: Vector, dir: Vector) -> impl DoubleEndedIterator<Item = Vector> {
        let (s, t) = match crate::cast::vector_to_char(dir) {
            'N' => (face * side_len, (face + E) * side_len - E),
            'E' => ((face + E) * side_len - E, (face + SE) * side_len - SE),
            'S' => ((face + SE) * side_len - SE, (face + S) * side_len - S),
            'W' => ((face + S) * side_len - S, face * side_len),
            _ => unreachable!(),
        };
        crate::grid::line_segment(s, t)
    }

    let mut res = HashMap::new();
    for (a, b) in sides {
        for (pos_a, pos_b) in edge(side_len, a.pos, a.dir).zip(edge(side_len, b.pos, b.dir).rev()) {
            res.insert(State::new(pos_a, a.dir), State::new(pos_b, b.dir * -1));
            res.insert(State::new(pos_b, b.dir), State::new(pos_a, a.dir * -1));
        }
    }

    res
}

fn part_(portals: Option<&HashMap<State, State>>, input: &str) -> i64 {
    let (board, instructions) = parse(input);

    let mut state = State::new(
        board
            .keys()
            .copied()
            .min_by_key(|&v| crate::grid::reading_ord_key(v))
            .unwrap(),
        E,
    );

    for instruction in instructions {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..n {
                    if let Some(s) = step(&board, portals, state) {
                        state = s;
                    } else {
                        break;
                    }
                }
            }
            Instruction::Turn(turn) => {
                state.dir = turn * state.dir;
            }
        }
    }

    let facing = match crate::cast::vector_to_char(state.dir) {
        'E' => 0,
        'S' => 1,
        'W' => 2,
        'N' => 3,
        _ => unreachable!(),
    };

    1000 * (state.pos.y + 1) + 4 * (state.pos.x + 1) + facing
}

pub fn part1(input: &str) -> i64 {
    part_(None, input)
}

pub fn part2(input: &str) -> i64 {
    part_(
        Some(&portals(
            50,
            &[
                (State::new(vector![1, 0], N), State::new(vector![0, 3], W)),
                (State::new(vector![1, 0], W), State::new(vector![0, 2], W)),
                (State::new(vector![2, 0], N), State::new(vector![0, 3], S)),
                (State::new(vector![2, 0], E), State::new(vector![1, 2], E)),
                (State::new(vector![2, 0], S), State::new(vector![1, 1], E)),
                (State::new(vector![1, 1], W), State::new(vector![0, 2], N)),
                (State::new(vector![1, 2], S), State::new(vector![0, 3], E)),
            ],
        )),
        input,
    )
}

pub fn tests() {
    let example = concat!(
        "        ...#\n",
        "        .#..\n",
        "        #...\n",
        "        ....\n",
        "...#.......#\n",
        "........#...\n",
        "..#....#....\n",
        "..........#.\n",
        "        ...#....\n",
        "        .....#..\n",
        "        .#......\n",
        "        ......#.\n",
        "\n",
        "10R5L5R10L4R5L5\n",
    );
    assert_eq!(part1(example), 6032);
    assert_eq!(
        part_(
            Some(&portals(
                4,
                &[
                    (State::new(vector![2, 0], N), State::new(vector![0, 1], N)),
                    (State::new(vector![2, 0], E), State::new(vector![3, 2], E)),
                    (State::new(vector![2, 0], W), State::new(vector![1, 1], N)),
                    (State::new(vector![0, 1], W), State::new(vector![3, 2], S)),
                    (State::new(vector![0, 1], S), State::new(vector![2, 2], S)),
                    (State::new(vector![1, 1], S), State::new(vector![2, 2], E)),
                    (State::new(vector![2, 1], E), State::new(vector![3, 2], N)),
                ]
            )),
            example
        ),
        5031
    );
}
