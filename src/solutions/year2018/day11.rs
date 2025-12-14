use crate::{
    grid::{Grid, NW, Vector},
    part::Part,
};

fn cell(serial: i64, pos: Vector) -> i64 {
    ((pos.x + 10) * pos.y + serial) * (pos.x + 10) / 100 % 10 - 5
}

// https://en.wikipedia.org/wiki/Summed-area_table
fn summed_area_table(serial: i64) -> Grid<i64> {
    let mut res = Grid::new(0, [301, 301]);
    for x in 1..=300 {
        for y in 1..=300 {
            res[[x, y]] = cell(serial, Vector::new(x, y)) + res[[x - 1, y]] + res[[x, y - 1]]
                - res[[x - 1, y - 1]];
        }
    }
    res
}

fn square(sat: &Grid<i64>, pos: Vector, size: i64) -> i64 {
    let base = pos + NW;
    sat[base] + sat[base + Vector::new(size, size)]
        - sat[base + Vector::new(size, 0)]
        - sat[base + Vector::new(0, size)]
}

fn part_(part: Part, input: &str) -> (Vector, i64) {
    let sat = summed_area_table(input.parse().unwrap());
    (match part {
        Part::One => 3..=3,
        Part::Two => 1..=300,
    })
    .flat_map(|size| {
        (1..=301 - size).flat_map(move |x| (1..=301 - size).map(move |y| (Vector::new(x, y), size)))
    })
    .max_by_key(|&(pos, size)| square(&sat, pos, size))
    .unwrap()
}

pub fn part1(input: &str) -> String {
    let (v, _) = part_(Part::One, input);
    format!("{},{}", v.x, v.y)
}

pub fn part2(input: &str) -> String {
    let (v, size) = part_(Part::Two, input);
    format!("{},{},{}", v.x, v.y, size)
}

pub fn tests() {
    assert_eq!(cell(8, Vector::new(3, 5)), 4);
    assert_eq!(cell(57, Vector::new(122, 79)), -5);
    assert_eq!(cell(39, Vector::new(217, 196)), 0);
    assert_eq!(cell(71, Vector::new(101, 153)), 4);

    assert_eq!(part1("18"), "33,45");
    assert_eq!(part1("42"), "21,61");

    assert_eq!(part2("18"), "90,269,16");
    assert_eq!(part2("42"), "232,251,12");
}
