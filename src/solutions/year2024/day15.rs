use crate::{
    grid::{Grid, Vector, E, W},
    search,
};

fn parse(input: &str) -> (Grid<char>, impl Iterator<Item = Vector> + '_) {
    let (warehouse, dirs) = input.split_once("\n\n").unwrap();
    (
        Grid::parse(warehouse, |_, c| c),
        dirs.chars()
            .filter(|&c| "^>v<".contains(c))
            .map(crate::cast::char_to_vector),
    )
}

fn expand(state: &Grid<char>) -> Grid<char> {
    let scale = Vector::new(2, 1);
    let mut res = Grid::new('.', state.size.component_mul(&scale));
    for (v, tile) in state {
        let v = v.component_mul(&scale);
        match tile {
            '.' => (),
            '#' => {
                res[v] = '#';
                res[v + E] = '#';
            }
            'O' => {
                res[v] = '[';
                res[v + E] = ']';
            }
            '@' => res[v] = '@',
            _ => unreachable!(),
        }
    }
    res
}

fn tick(state: &mut Grid<char>, dir: Vector) {
    let objects: Vec<Vector> = search::breadth_first(
        state.keys().find(|&v| state[v] == '@').unwrap(),
        |&v, push| match state[v] {
            '@' | 'O' => {
                push(v + dir);
            }
            '[' => {
                push(v + dir);
                push(v + E);
            }
            ']' => {
                push(v + dir);
                push(v + W);
            }
            _ => (),
        },
        search::id_filter(),
    )
    .filter(|&v| "@O[]".contains(state[v]))
    .collect();

    if objects.iter().all(|&v| state[v + dir] != '#') {
        let mut next = state.clone();
        for &v in &objects {
            next[v] = '.';
        }
        for &v in &objects {
            next[v + dir] = state[v];
        }
        *state = next;
    }
}

fn score(state: &Grid<char>) -> i64 {
    state
        .iter()
        .filter(|&(_, &tile)| "O[".contains(tile))
        .map(|(v, _)| v.x + 100 * v.y)
        .sum()
}

fn part_(mut state: Grid<char>, dirs: impl Iterator<Item = Vector>) -> i64 {
    for dir in dirs {
        tick(&mut state, dir);
    }
    score(&state)
}

pub fn part1(input: &str) -> i64 {
    let (state, dirs) = parse(input);
    part_(state, dirs)
}

pub fn part2(input: &str) -> i64 {
    let (state, dirs) = parse(input);
    part_(expand(&state), dirs)
}

pub fn tests() {
    let small_example = [
        "########",
        "#..O.O.#",
        "##@.O..#",
        "#...O..#",
        "#.#.O..#",
        "#...O..#",
        "#......#",
        "########",
        "",
        "<^^>>>vv<v>>v<<",
    ]
    .join("\n");
    let big_example = [
        "##########",
        "#..O..O.O#",
        "#......O.#",
        "#.OO..O.O#",
        "#..O@..O.#",
        "#O#..O...#",
        "#O..O..O.#",
        "#.OO.O.OO#",
        "#....O...#",
        "##########",
        "",
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ]
    .join("\n");
    assert_eq!(part1(&small_example), 2028);
    assert_eq!(part1(&big_example), 10092);
    assert_eq!(part2(&big_example), 9021);
}
