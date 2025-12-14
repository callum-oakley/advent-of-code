use std::collections::HashSet;

use nalgebra::vector;

use crate::grid::{Adjacent, SE, Vector};

#[derive(Clone, Copy)]
struct Rectangle {
    a: Vector,
    b: Vector,
}

impl Rectangle {
    fn area(&self) -> i64 {
        ((self.a - self.b).abs() + SE).product()
    }

    fn perimeter(&self) -> impl Iterator<Item = Vector> {
        let min_x = self.a.x.min(self.b.x);
        let max_x = self.a.x.max(self.b.x);
        let min_y = self.a.y.min(self.b.y);
        let max_y = self.a.y.max(self.b.y);
        crate::grid::line_segment(vector![min_x, min_y], vector![min_x, max_y - 1])
            .chain(crate::grid::line_segment(
                vector![min_x, max_y],
                vector![max_x - 1, max_y],
            ))
            .chain(crate::grid::line_segment(
                vector![max_x, max_y],
                vector![max_x, min_y + 1],
            ))
            .chain(crate::grid::line_segment(
                vector![max_x, min_y],
                vector![min_x + 1, min_y],
            ))
    }
}

struct Decompressor {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

impl Decompressor {
    fn decompress(&self, tile: Vector) -> Vector {
        Vector::new(
            self.xs[usize::try_from(tile.x).unwrap()],
            self.ys[usize::try_from(tile.y).unwrap()],
        )
    }

    fn decompress_rectangle(&self, rectangle: Rectangle) -> Rectangle {
        Rectangle {
            a: self.decompress(rectangle.a),
            b: self.decompress(rectangle.b),
        }
    }
}

fn compress(tiles: &mut [Vector]) -> Decompressor {
    let mut xs: Vec<_> = tiles.iter().map(|&t| t.x).collect();
    let mut ys: Vec<_> = tiles.iter().map(|&t| t.y).collect();
    xs.sort_unstable();
    ys.sort_unstable();

    for tile in tiles {
        tile.x = i64::try_from(xs.iter().position(|&x| x == tile.x).unwrap()).unwrap();
        tile.y = i64::try_from(ys.iter().position(|&y| y == tile.y).unwrap()).unwrap();
    }

    Decompressor { xs, ys }
}

fn parse(input: &str) -> Vec<Vector> {
    input.lines().map(crate::cast::str_to_vector).collect()
}

fn rectangles(red_tiles: &[Vector]) -> Vec<Rectangle> {
    crate::combinatorics::combinations(2, red_tiles)
        .map(|pair| Rectangle {
            a: *pair[0],
            b: *pair[1],
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let red_tiles = parse(input);
    let rectangles = rectangles(&red_tiles);
    rectangles.into_iter().map(|r| r.area()).max().unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut red_tiles = parse(input);
    let decompressor = compress(&mut red_tiles);

    let mut tiles = red_tiles.iter().copied().collect::<HashSet<_>>();
    for i in 0..red_tiles.len() {
        tiles.extend(crate::grid::line_segment(
            red_tiles[i],
            red_tiles[(i + 1) % red_tiles.len()],
        ));
    }

    let start = red_tiles
        .iter()
        .min_by_key(|&&tile| crate::grid::reading_ord_key(tile))
        .unwrap()
        + SE;
    assert!(!tiles.contains(&start));

    tiles.extend(
        crate::search::breadth_first(
            start,
            |tile, push| {
                tile.adjacent4()
                    .filter(|&tile| !tiles.contains(&tile))
                    .for_each(push);
            },
            crate::search::id_filter(),
        )
        .collect::<Vec<_>>(),
    );

    let rectangles = rectangles(&red_tiles);
    rectangles
        .into_iter()
        // Need only check that the perimeter of a given rectangle is contained in tiles.
        .filter(|r| r.perimeter().all(|tile| tiles.contains(&tile)))
        .map(|r| decompressor.decompress_rectangle(r).area())
        .max()
        .unwrap()
}

pub fn tests() {
    let example = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
    assert_eq!(part1(example), 50);
    assert_eq!(part2(example), 24);
}
