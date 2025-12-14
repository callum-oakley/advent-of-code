use crate::grid::{E, Grid, N, S, Vector, W};

fn line_of_sight(g: &Grid<u8>, start: Vector, dir: Vector) -> impl Iterator<Item = u8> {
    std::iter::successors(Some(start + dir), move |v| Some(v + dir))
        .map_while(|v| g.get(v).copied())
}

fn visible(g: &Grid<u8>, tree: Vector) -> bool {
    [N, E, S, W]
        .into_iter()
        .any(|dir| line_of_sight(g, tree, dir).all(|h| h < g[tree]))
}

fn scenic_score(g: &Grid<u8>, tree: Vector) -> usize {
    [N, E, S, W]
        .into_iter()
        .map(|dir| {
            let mut viewing_distance = 0;
            for h in line_of_sight(g, tree, dir) {
                viewing_distance += 1;
                if h >= g[tree] {
                    // We can see this tree but we can't see beyond it. It still contributes to the
                    // viewing distance.
                    break;
                }
            }
            viewing_distance
        })
        .product()
}

pub fn part1(input: &str) -> usize {
    let g = Grid::parse(input, |_, c| u8::try_from(c.to_digit(10).unwrap()).unwrap());
    g.keys().filter(|&tree| visible(&g, tree)).count()
}

pub fn part2(input: &str) -> usize {
    let g = Grid::parse(input, |_, c| u8::try_from(c.to_digit(10).unwrap()).unwrap());
    g.keys().map(|tree| scenic_score(&g, tree)).max().unwrap()
}

pub fn tests() {
    let example = "30373\n25512\n65332\n33549\n35390";
    assert_eq!(part1(example), 21);
    assert_eq!(part2(example), 8);
}
