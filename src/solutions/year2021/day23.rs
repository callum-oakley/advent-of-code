use std::{collections::BTreeMap, sync::LazyLock};

use crate::{grid::Vector, lex_ord::LexOrd};

#[derive(Clone)]
struct State {
    amphipods: BTreeMap<LexOrd<Vector>, Option<char>>,
    energy: usize,
}

fn parse(input: &str) -> State {
    let mut amphipods = BTreeMap::new();
    for (v, c) in crate::grid::scan(input) {
        match c {
            'A'..='D' => {
                amphipods.insert(LexOrd(v), Some(c));
            }
            '.' => {
                amphipods.insert(LexOrd(v), None);
            }
            _ => {}
        }
    }
    State {
        amphipods,
        energy: 0,
    }
}

fn energy(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

static HALLS: LazyLock<[Vector; 7]> =
    LazyLock::new(|| [1, 2, 4, 6, 8, 10, 11].map(|x| Vector::new(x, 1)));

fn in_hallway(v: Vector) -> bool {
    v.y == 1
}

fn room_x(amphipod: char) -> i64 {
    match amphipod {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => unreachable!(),
    }
}

fn rooms(
    amphipods: &BTreeMap<LexOrd<Vector>, Option<char>>,
    amphipod: char,
) -> impl Iterator<Item = Vector> {
    let x = room_x(amphipod);
    (2..i64::MAX)
        .map(move |y| Vector::new(x, y))
        .take_while(|&v| amphipods.contains_key(&LexOrd(v)))
}

fn needs_to_move(amphipods: &BTreeMap<LexOrd<Vector>, Option<char>>, v: Vector) -> bool {
    // An amphipod needs to move if it's in the wrong room or if there are any amphipods of a
    // different type below it.
    let amphipod = amphipods[&LexOrd(v)].unwrap();
    room_x(amphipod) != v.x
        || rooms(amphipods, amphipod)
            .any(|room| room.y > v.y && amphipods[&LexOrd(room)].is_some_and(|a| a != amphipod))
}

fn lowest_free_room(
    amphipods: &BTreeMap<LexOrd<Vector>, Option<char>>,
    amphipod: char,
) -> Option<Vector> {
    // If any of the amphipods in the room are of the wrong type, then the room needs to be emptied
    // first so there are no free rooms.
    if rooms(amphipods, amphipod)
        .any(|room| amphipods[&LexOrd(room)].is_some_and(|a| a != amphipod))
    {
        None
    } else {
        rooms(amphipods, amphipod)
            .filter(|&room| amphipods[&LexOrd(room)].is_none())
            .max_by_key(|v| v.y)
    }
}

fn path_from_room_to_hall(room: Vector, hall: Vector) -> Vec<Vector> {
    fn range(a: i64, b: i64) -> Box<dyn Iterator<Item = i64>> {
        if a > b {
            Box::new((b..=a).rev())
        } else {
            Box::new(a..=b)
        }
    }

    assert!(!in_hallway(room));
    assert!(in_hallway(hall));

    let mut res = Vec::new();
    for y in range(room.y, 2) {
        res.push(Vector::new(room.x, y));
    }
    for x in range(room.x, hall.x) {
        res.push(Vector::new(x, 1));
    }

    res
}

fn path_from_hall_to_room(hall: Vector, room: Vector) -> Vec<Vector> {
    let mut res = path_from_room_to_hall(room, hall);
    res.reverse();
    res
}

fn next_state(state: &State, path: &[Vector]) -> State {
    let mut res = state.clone();
    let a = res.amphipods[&LexOrd(path[0])].unwrap();
    res.amphipods.insert(LexOrd(path[0]), None);
    res.amphipods.insert(LexOrd(path[path.len() - 1]), Some(a));
    res.energy += (path.len() - 1) * energy(a);
    res
}

fn adjacent(state: &State, push: &mut dyn FnMut(State)) {
    for (&LexOrd(v), amphipod) in &state.amphipods {
        if let Some(amphipod) = amphipod {
            if in_hallway(v) {
                if let Some(room) = lowest_free_room(&state.amphipods, *amphipod) {
                    let path = path_from_hall_to_room(v, room);
                    if path[1..]
                        .iter()
                        .all(|&v| state.amphipods[&LexOrd(v)].is_none())
                    {
                        push(next_state(state, &path));
                    }
                }
            } else if needs_to_move(&state.amphipods, v) {
                for &hall in HALLS.iter() {
                    let path = path_from_room_to_hall(v, hall);
                    if path[1..]
                        .iter()
                        .all(|&v| state.amphipods[&LexOrd(v)].is_none())
                    {
                        push(next_state(state, &path));
                    }
                }
            }
        }
    }
}

fn is_organized(state: &State) -> bool {
    ['A', 'B', 'C', 'D'].iter().all(|&amphipod| {
        rooms(&state.amphipods, amphipod)
            .all(|v| state.amphipods[&LexOrd(v)].is_some_and(|a| a == amphipod))
    })
}

pub fn part1(input: &str) -> usize {
    crate::search::dijkstra(
        parse(input),
        adjacent,
        crate::search::hash_filter(|state: &State| state.amphipods.clone()),
        |state| state.energy,
    )
    .find(is_organized)
    .unwrap()
    .energy
}

pub fn part2(input: &str) -> usize {
    let mut lines: Vec<&str> = input.split('\n').collect();
    lines.insert(3, "  #D#C#B#A#\n  #D#B#A#C#");
    part1(&lines.join("\n"))
}

pub fn tests() {
    let example = concat!(
        "#############\n",
        "#...........#\n",
        "###B#C#B#D###\n",
        "  #A#D#C#A#  \n",
        "  #########  \n",
    );
    assert_eq!(part1(example), 12521);
    assert_eq!(part2(example), 44169);
}
