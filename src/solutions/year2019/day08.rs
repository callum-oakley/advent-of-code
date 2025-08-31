use crate::grid::{Grid, Vector};

const IMAGE_SIZE: Vector = Vector::new(25, 6);

#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Black,
    White,
    Transparent,
}

fn parse(input: &str) -> Vec<Grid<Pixel>> {
    let mut res = Vec::new();
    let mut chars = input.chars();
    for _ in 0..input.len() / usize::try_from(IMAGE_SIZE.y * IMAGE_SIZE.x).unwrap() {
        let mut layer = Grid::new(Pixel::Transparent, IMAGE_SIZE);
        for y in 0..IMAGE_SIZE.y {
            for x in 0..IMAGE_SIZE.x {
                layer[[x, y]] = match chars.next().unwrap() {
                    '0' => Pixel::Black,
                    '1' => Pixel::White,
                    '2' => Pixel::Transparent,
                    _ => unreachable!(),
                }
            }
        }
        res.push(layer);
    }
    res
}

pub fn part1(input: &str) -> usize {
    let layer = parse(input)
        .into_iter()
        .min_by_key(|layer| layer.values().filter(|&&p| p == Pixel::Black).count())
        .unwrap();
    layer.values().filter(|&&p| p == Pixel::White).count()
        * layer.values().filter(|&&p| p == Pixel::Transparent).count()
}

pub fn part2(input: &str) -> &str {
    let layers = parse(input);
    let mut image = Grid::new(false, IMAGE_SIZE);
    for layer in layers.iter().rev() {
        for (pos, &pixel) in layer {
            match pixel {
                Pixel::Black => {
                    image[pos] = false;
                }
                Pixel::White => {
                    image[pos] = true;
                }
                Pixel::Transparent => {}
            }
        }
    }
    crate::ocr::parse(&crate::cast::bool_grid_to_string(&image))
}
