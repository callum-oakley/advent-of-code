use crate::grid::{Adjacent, Grid, NE, NW, SE, SW, Vector, Z};

pub fn part1(input: &str) -> usize {
    let grid = Grid::parse(input, |_, c| c);
    let word_at = |v: Vector, dir: Vector| -> String {
        (0..4).filter_map(|i| grid.get(v + i * dir)).collect()
    };
    grid.keys()
        .flat_map(|v| Z.adjacent8().map(move |dir| (v, dir)))
        .filter(|&(v, dir)| word_at(v, dir) == "XMAS")
        .count()
}

pub fn part2(input: &str) -> usize {
    let grid = Grid::parse(input, |_, c| c);
    let xmas_at = |v: Vector| -> bool {
        [[NW, Z, SE], [NE, Z, SW]]
            .iter()
            .map(|dirs| {
                dirs.iter()
                    .filter_map(|dir| grid.get(v + dir))
                    .collect::<String>()
            })
            .all(|diagonal| ["MAS", "SAM"].contains(&diagonal.as_str()))
    };
    grid.keys().filter(|&v| xmas_at(v)).count()
}

pub fn tests() {
    let example = [
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ]
    .join("\n");
    assert_eq!(part1(&example), 18);
    assert_eq!(part2(&example), 9);
}
