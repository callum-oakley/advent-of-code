use nalgebra::{Vector2, Vector3, matrix, vector};

use crate::combinatorics;

#[derive(Clone, Copy)]
struct Stone {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

fn xy_intersection(s1: Stone, s2: Stone) -> Option<(f64, f64, Vector2<f64>)> {
    // Using the equations from
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
    // re-arrange s = p + t * v to s = p + t * (v + p - p)
    let x1 = s1.p.x;
    let y1 = s1.p.y;
    let x2 = (s1.v + s1.p).x;
    let y2 = (s1.v + s1.p).y;
    let x3 = s2.p.x;
    let y3 = s2.p.y;
    let x4 = (s2.v + s2.p).x;
    let y4 = (s2.v + s2.p).y;

    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denominator == 0. {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;

    Some((t, u, Vector2::new(x1 + t * (x2 - x1), y1 + t * (y2 - y1))))
}

fn parse(input: &str) -> impl Iterator<Item = Stone> + '_ {
    input.trim().lines().map(|l| {
        let (p, v) = l.split_once('@').unwrap();
        Stone {
            p: Vector3::from_iterator(crate::cast::str_to_ints(p)),
            v: Vector3::from_iterator(crate::cast::str_to_ints(v)),
        }
    })
}

fn part1_(low: f64, high: f64, input: &str) -> usize {
    let stones = parse(input).collect::<Vec<_>>();
    combinatorics::combinations(2, &stones)
        .filter_map(|pair| xy_intersection(*pair[0], *pair[1]))
        .filter(|&(t, u, p)| {
            t >= 0. && u >= 0. && low <= p.x && p.x <= high && low <= p.y && p.y <= high
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    part1_(2e14, 4e14, input)
}

#[expect(clippy::cast_possible_truncation)]
pub fn part2(input: &str) -> i64 {
    let mut stones = parse(input);
    let s0 = stones.next().unwrap();
    let s1 = stones.next().unwrap();
    let s2 = stones.next().unwrap();
    let s3 = stones.next().unwrap();

    // Following the explanation here:
    // https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/
    //
    // Let the first four stones be s0, s1, s2, s3 and the rock be r, then
    //
    // r.p + t * r.v == si.p + t * si.v
    // r.p - si.p == -t * (r.v - si.v)
    //
    // so r.p - si.p and r.v - si.v are parallel and
    //
    // (r.p - si.p) x (r.v - si.v) == 0
    // r.p x r.v + si.v x r.p - si.p x r.v + si.p x si.v == 0
    //
    // Subtracting the equation for i = 1 from i = 0, and i = 3 from i = 2
    //
    // (s0.v - s1.v) x r.p + (s1.p - s0.p) x r.v + s0.p x s0.v + s1.v x s1.p == 0
    // (s2.v - s3.v) x r.p + (s3.p - s2.p) x r.v + s2.p x s2.v + s3.v x s3.p == 0
    //
    // which is a linear system of 6 equations with 6 unknowns
    //
    // (s0.v - s1.v).y * r.p.z - (s0.v - s1.v).z * r.p.y +
    //     (s1.p - s0.p).y * r.v.z - (s1.p - s0.p).z * r.v.y +
    //     s0.p.y * s0.v.z - s0.p.z * s0.v.y + s1.v.y * s1.p.z - s1.v.z * s1.p.y == 0
    // (s0.v - s1.v).z * r.p.x - (s0.v - s1.v).x * r.p.z +
    //     (s1.p - s0.p).z * r.v.x - (s1.p - s0.p).x * r.v.z +
    //     s0.p.z * s0.v.x - s0.p.x * s0.v.z + s1.v.z * s1.p.x - s1.v.x * s1.p.z == 0
    // (s0.v - s1.v).x * r.p.y - (s0.v - s1.v).y * r.p.x +
    //     (s1.p - s0.p).x * r.v.y - (s1.p - s0.p).y * r.v.x +
    //     s0.p.x * s0.v.y - s0.p.y * s0.v.x + s1.v.x * s1.p.y - s1.v.y * s1.p.x == 0
    // (s2.v - s3.v).y * r.p.z - (s2.v - s3.v).z * r.p.y +
    //     (s3.p - s2.p).y * r.v.z - (s3.p - s2.p).z * r.v.y +
    //     s2.p.y * s2.v.z - s2.p.z * s2.v.y + s3.v.y * s3.p.z - s3.v.z * s3.p.y == 0
    // (s2.v - s3.v).z * r.p.x - (s2.v - s3.v).x * r.p.z +
    //     (s3.p - s2.p).z * r.v.x - (s3.p - s2.p).x * r.v.z +
    //     s2.p.z * s2.v.x - s2.p.x * s2.v.z + s3.v.z * s3.p.x - s3.v.x * s3.p.z == 0
    // (s2.v - s3.v).x * r.p.y - (s2.v - s3.v).y * r.p.x +
    //     (s3.p - s2.p).x * r.v.y - (s3.p - s2.p).y * r.v.x +
    //     s2.p.x * s2.v.y - s2.p.y * s2.v.x + s3.v.x * s3.p.y - s3.v.y * s3.p.x == 0

    // columns correspond to r.p.x, r.p.y, r.p.z, r.v.x, r.v.y, r.v.z
    let m = matrix![
        0., - (s0.v - s1.v).z, (s0.v - s1.v).y, 0., - (s1.p - s0.p).z, (s1.p - s0.p).y;
        (s0.v - s1.v).z, 0., - (s0.v - s1.v).x, (s1.p - s0.p).z, 0., - (s1.p - s0.p).x;
        - (s0.v - s1.v).y, (s0.v - s1.v).x, 0., - (s1.p - s0.p).y, (s1.p - s0.p).x, 0.;
        0., - (s2.v - s3.v).z, (s2.v - s3.v).y, 0., - (s3.p - s2.p).z, (s3.p - s2.p).y;
        (s2.v - s3.v).z, 0., - (s2.v - s3.v).x, (s3.p - s2.p).z, 0., - (s3.p - s2.p).x;
        - (s2.v - s3.v).y, (s2.v - s3.v).x, 0., - (s3.p - s2.p).y, (s3.p - s2.p).x, 0.;
    ];

    let b = -vector![
        s0.p.y * s0.v.z - s0.p.z * s0.v.y + s1.v.y * s1.p.z - s1.v.z * s1.p.y,
        s0.p.z * s0.v.x - s0.p.x * s0.v.z + s1.v.z * s1.p.x - s1.v.x * s1.p.z,
        s0.p.x * s0.v.y - s0.p.y * s0.v.x + s1.v.x * s1.p.y - s1.v.y * s1.p.x,
        s2.p.y * s2.v.z - s2.p.z * s2.v.y + s3.v.y * s3.p.z - s3.v.z * s3.p.y,
        s2.p.z * s2.v.x - s2.p.x * s2.v.z + s3.v.z * s3.p.x - s3.v.x * s3.p.z,
        s2.p.x * s2.v.y - s2.p.y * s2.v.x + s3.v.x * s3.p.y - s3.v.y * s3.p.x,
    ];

    let r = m.qr().solve(&b).unwrap();

    (r[0].round() + r[1].round() + r[2].round()) as i64
}

pub fn tests() {
    let example = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    ";
    assert_eq!(part1_(7., 27., example), 2);
    assert_eq!(part2(example), 47);
}
