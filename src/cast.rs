use std::{collections::HashSet, sync::LazyLock};

use nalgebra::vector;
use regex::Regex;

use crate::grid::{Bounds, E, Grid, LEFT, N, RIGHT, S, Turn, Vector, Vector3, Vector4, W, Z};

pub fn string_to_ints<N>(s: &str) -> impl Iterator<Item = N>
where
    N: std::str::FromStr,
    N::Err: std::fmt::Debug,
{
    static INTS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-?\d+").unwrap());
    INTS.find_iter(s).map(|m| m.as_str().parse().unwrap())
}

pub fn string_to_vector(s: &str) -> Vector {
    if s.len() == 1 {
        char_to_vector(s.chars().next().unwrap())
    } else {
        Vector::from_iterator(string_to_ints(s))
    }
}

pub fn string_to_vector3(s: &str) -> Vector3 {
    Vector3::from_iterator(crate::cast::string_to_ints(s))
}

pub fn string_to_vector4(s: &str) -> Vector4 {
    Vector4::from_iterator(crate::cast::string_to_ints(s))
}

pub fn char_to_vector(c: char) -> Vector {
    match c {
        'N' | 'U' | '^' => N,
        'E' | 'R' | '>' => E,
        'S' | 'D' | 'v' => S,
        'W' | 'L' | '<' => W,
        _ => panic!("don't know how to convert {c} into a vector"),
    }
}

pub fn vector_to_char(v: Vector) -> char {
    if v == N {
        'N'
    } else if v == W {
        'W'
    } else if v == Z {
        'Z'
    } else if v == E {
        'E'
    } else if v == S {
        'S'
    } else {
        panic!("don't know how to convert {v} into a char")
    }
}

pub fn string_to_turn(s: &str) -> Turn {
    char_to_turn(s.chars().next().unwrap())
}

pub fn char_to_turn(c: char) -> Turn {
    match c {
        'L' => LEFT,
        'R' => RIGHT,
        _ => panic!("don't know how to convert {c} into a turn"),
    }
}

pub fn bool_grid_to_string(g: &Grid<bool>) -> String {
    let mut res = String::new();
    for y in 0..g.size.y {
        for x in 0..g.size.x {
            res.push(if g[vector![x, y]] { '#' } else { '.' });
        }
        res.push('\n');
    }
    res
}

pub fn vector_hash_set_to_string(g: &HashSet<Vector>) -> String {
    let bounds = Bounds::from(g);
    let mut res = String::new();
    for y in bounds.min.y..=bounds.max.y {
        for x in bounds.min.x..=bounds.max.x {
            res.push(if g.contains(&vector![x, y]) { '#' } else { '.' });
        }
        res.push('\n');
    }
    res
}
