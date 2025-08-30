use std::{collections::HashMap, sync::LazyLock};

use crate::{
    grid::{Adjacent, Bounds, Grid, Vector, E, N, S, W, Z},
    uniq::Uniq,
};

fn flip<T: Clone>(grid: &Grid<T>) -> Grid<T> {
    let mut res = grid.clone();
    for y in 0..grid.size.y {
        for x in 0..grid.size.x {
            res[[x, y]] = grid[[grid.size.x - 1 - x, y]].clone();
        }
    }
    res
}

fn rotate<T: Clone>(grid: &Grid<T>) -> Grid<T> {
    let mut res = grid.clone();
    for y in 0..grid.size.y {
        for x in 0..grid.size.x {
            res[[x, y]] = grid[[y, grid.size.x - 1 - x]].clone();
        }
    }
    res
}

fn orientations<T: Clone>(mut grid: Grid<T>) -> Vec<Grid<T>> {
    let mut res = Vec::new();
    for _ in 0..2 {
        for _ in 0..3 {
            let next = rotate(&grid);
            res.push(grid);
            grid = next;
        }
        let next = flip(&grid);
        res.push(grid);
        grid = next;
    }
    res
}

#[derive(Clone)]
struct Tile {
    id: u64,
    image: Grid<char>,
}

impl Tile {
    fn edge(&self, dir: Vector) -> Vec<char> {
        if dir == N {
            (0..self.image.size.x).map(|x| self.image[[x, 0]]).collect()
        } else if dir == E {
            (0..self.image.size.y)
                .map(|y| self.image[[self.image.size.x - 1, y]])
                .collect()
        } else if dir == S {
            (0..self.image.size.x)
                .map(|x| self.image[[x, self.image.size.y - 1]])
                .collect()
        } else if dir == W {
            (0..self.image.size.y).map(|y| self.image[[0, y]]).collect()
        } else {
            unreachable!()
        }
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|s| {
            let (header, image) = s.split_once('\n').unwrap();
            Tile {
                id: header
                    .trim_start_matches("Tile ")
                    .trim_end_matches(':')
                    .parse()
                    .unwrap(),
                image: Grid::parse(image, |_, c| c),
            }
        })
        .collect()
}

// Assemmble tiles in to the right positions.
fn assemble(tiles: Vec<Tile>) -> HashMap<Vector, Tile> {
    let mut free: Vec<Vec<Tile>> = tiles
        .into_iter()
        .map(|tile| {
            orientations(tile.image)
                .into_iter()
                .map(|image| Tile { id: tile.id, image })
                .collect()
        })
        .collect();

    let mut fixed = HashMap::from([(Z, free.pop().unwrap().pop().unwrap())]);
    while !free.is_empty() {
        let (v, i, j) = fixed
            .keys()
            .flat_map(|&v| v.adjacent4())
            .uniq()
            .filter(|v| !fixed.contains_key(v))
            .find_map(|v| {
                free.iter().enumerate().find_map(|(i, tiles)| {
                    tiles.iter().enumerate().find_map(|(j, tile)| {
                        if [N, E, S, W].into_iter().any(|dir| {
                            fixed
                                .get(&(v + dir))
                                .is_some_and(|t| tile.edge(dir) == t.edge(-dir))
                        }) {
                            Some((v, i, j))
                        } else {
                            None
                        }
                    })
                })
            })
            .unwrap();
        fixed.insert(v, free.swap_remove(i).swap_remove(j));
    }

    fixed
}

// Stitch assembled tiles together in to a single image.
fn stitch(tiles: HashMap<Vector, Tile>) -> Grid<char> {
    let trimmed_tiles: HashMap<_, _> = tiles
        .into_iter()
        .map(|(v, tile)| {
            (v, {
                let mut trimmed_tile = Grid::new('.', tile.image.size - Vector::new(2, 2));
                for y in 0..trimmed_tile.size.y {
                    for x in 0..trimmed_tile.size.x {
                        trimmed_tile[[x, y]] = tile.image[[x + 1, y + 1]];
                    }
                }
                trimmed_tile
            })
        })
        .collect();

    let trimmed_tiles_bounds = Bounds::from(&trimmed_tiles);
    let tile_size = trimmed_tiles[&Z].size;

    let mut res = Grid::new('.', trimmed_tiles_bounds.size().component_mul(&tile_size));
    for (u, tile) in trimmed_tiles {
        for (v, &c) in &tile {
            res[(u - trimmed_tiles_bounds.min).component_mul(&tile_size) + v] = c;
        }
    }
    res
}

pub fn part1(input: &str) -> u64 {
    let fixed = assemble(parse(input));
    let bounds = Bounds::from(&fixed);
    fixed[&bounds.min].id
        * fixed[&Vector::new(bounds.min.x, bounds.max.y)].id
        * fixed[&Vector::new(bounds.max.x, bounds.min.y)].id
        * fixed[&bounds.max].id
}

pub fn part2(input: &str) -> usize {
    static SEA_MONSTER: LazyLock<Grid<char>> = LazyLock::new(|| {
        Grid::parse(
            &[
                "                  # ",
                "#    ##    ##    ###",
                " #  #  #  #  #  #   ",
            ]
            .join("\n"),
            |_, c| c,
        )
    });

    fn is_sea_monster(image: &Grid<char>, u: Vector) -> bool {
        SEA_MONSTER
            .iter()
            .all(|(v, &c)| c == ' ' || image.get(u + v) == Some(&'#'))
    }

    let image = stitch(assemble(parse(input)));

    let hash_count = image.values().filter(|&&c| c == '#').count();

    let sea_monster_count = orientations(image)
        .into_iter()
        .map(|image| image.keys().filter(|&u| is_sea_monster(&image, u)).count())
        .max()
        .unwrap();

    hash_count - sea_monster_count * SEA_MONSTER.values().filter(|&&c| c == '#').count()
}

#[expect(clippy::too_many_lines)]
pub fn tests() {
    let example = [
        "Tile 2311:",
        "..##.#..#.",
        "##..#.....",
        "#...##..#.",
        "####.#...#",
        "##.##.###.",
        "##...#.###",
        ".#.#.#..##",
        "..#....#..",
        "###...#.#.",
        "..###..###",
        "",
        "Tile 1951:",
        "#.##...##.",
        "#.####...#",
        ".....#..##",
        "#...######",
        ".##.#....#",
        ".###.#####",
        "###.##.##.",
        ".###....#.",
        "..#.#..#.#",
        "#...##.#..",
        "",
        "Tile 1171:",
        "####...##.",
        "#..##.#..#",
        "##.#..#.#.",
        ".###.####.",
        "..###.####",
        ".##....##.",
        ".#...####.",
        "#.##.####.",
        "####..#...",
        ".....##...",
        "",
        "Tile 1427:",
        "###.##.#..",
        ".#..#.##..",
        ".#.##.#..#",
        "#.#.#.##.#",
        "....#...##",
        "...##..##.",
        "...#.#####",
        ".#.####.#.",
        "..#..###.#",
        "..##.#..#.",
        "",
        "Tile 1489:",
        "##.#.#....",
        "..##...#..",
        ".##..##...",
        "..#...#...",
        "#####...#.",
        "#..#.#.#.#",
        "...#.#.#..",
        "##.#...##.",
        "..##.##.##",
        "###.##.#..",
        "",
        "Tile 2473:",
        "#....####.",
        "#..#.##...",
        "#.##..#...",
        "######.#.#",
        ".#...#.#.#",
        ".#########",
        ".###.#..#.",
        "########.#",
        "##...##.#.",
        "..###.#.#.",
        "",
        "Tile 2971:",
        "..#.#....#",
        "#...###...",
        "#.#.###...",
        "##.##..#..",
        ".#####..##",
        ".#..####.#",
        "#..#.#..#.",
        "..####.###",
        "..#.#.###.",
        "...#.#.#.#",
        "",
        "Tile 2729:",
        "...#.#.#.#",
        "####.#....",
        "..#.#.....",
        "....#..#.#",
        ".##..##.#.",
        ".#.####...",
        "####.#.#..",
        "##.####...",
        "##..#.##..",
        "#.##...##.",
        "",
        "Tile 3079:",
        "#.#.#####.",
        ".#..######",
        "..#.......",
        "######....",
        "####.#..#.",
        ".#...#.##.",
        "#.#####.##",
        "..#.###...",
        "..#.......",
        "..#.###...",
    ]
    .join("\n");
    assert_eq!(part1(&example), 20_899_048_083_289);
    assert_eq!(part2(&example), 273);
}
