use std::collections::{HashMap, HashSet, VecDeque};

use crate::grid::Vector3;

type Brick = Vec<Vector3>;

fn parse(input: &str) -> (HashMap<usize, Brick>, HashMap<Vector3, usize>) {
    let bricks = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start: Vector3 = crate::cast::string_to_vector3(start);
            let end: Vector3 = crate::cast::string_to_vector3(end);
            let mut brick = vec![start];
            let mut pos = start;
            while pos != end {
                pos += (end - start) / (end - start).abs().sum();
                brick.push(pos);
            }
            brick
        })
        .enumerate()
        .collect::<HashMap<_, _>>();

    let mut cubes = HashMap::new();
    for (&i, brick) in &bricks {
        for &cube in brick {
            cubes.insert(cube, i);
        }
    }

    (bricks, cubes)
}

const UP: Vector3 = Vector3::new(0, 0, 1);

fn supports(
    bricks: &HashMap<usize, Brick>,
    cubes: &HashMap<Vector3, usize>,
    i: usize,
) -> HashSet<usize> {
    let mut res = HashSet::new();
    for &cube in &bricks[&i] {
        if let Some(&j) = cubes.get(&(cube + UP)) {
            if j != i {
                res.insert(j);
            }
        }
    }
    res
}

fn supported_by(
    bricks: &HashMap<usize, Brick>,
    cubes: &HashMap<Vector3, usize>,
    i: usize,
) -> HashSet<usize> {
    let mut res = HashSet::new();
    for &cube in &bricks[&i] {
        if let Some(&j) = cubes.get(&(cube - UP)) {
            if j != i {
                res.insert(j);
            }
        }
    }
    res
}

fn settle(
    bricks: &mut HashMap<usize, Brick>,
    cubes: &mut HashMap<Vector3, usize>,
) -> HashSet<usize> {
    let mut res = HashSet::new();

    let mut q = VecDeque::new();
    for &i in bricks.keys() {
        q.push_back(i);
    }

    while let Some(i) = q.pop_front() {
        if bricks[&i][0].z >= 2 && supported_by(bricks, cubes, i).is_empty() {
            for j in 0..bricks[&i].len() {
                for k in supports(bricks, cubes, i) {
                    q.push_back(k);
                }
                cubes.remove(&bricks[&i][j]);
                bricks.get_mut(&i).unwrap()[j] -= UP;
                cubes.insert(bricks[&i][j], i);
            }
            q.push_back(i);
            res.insert(i);
        }
    }

    res
}

pub fn part1(input: &str) -> usize {
    let (mut bricks, mut cubes) = parse(input);

    settle(&mut bricks, &mut cubes);

    bricks
        .keys()
        .filter(|&&i| {
            supports(&bricks, &cubes, i)
                .iter()
                .all(|&j| supported_by(&bricks, &cubes, j).len() >= 2)
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    let (mut bricks, mut cubes) = parse(input);

    settle(&mut bricks, &mut cubes);

    bricks
        .keys()
        .map(|&i| {
            let mut bricks = bricks.clone();
            let mut cubes = cubes.clone();
            for cube in &bricks[&i] {
                cubes.remove(cube);
            }
            bricks.remove(&i);
            settle(&mut bricks, &mut cubes).len()
        })
        .sum()
}

pub fn tests() {
    let example = [
        "1,0,1~1,2,1",
        "0,0,2~2,0,2",
        "0,2,3~2,2,3",
        "0,0,4~0,2,4",
        "2,0,5~2,2,5",
        "0,1,6~2,1,6",
        "1,1,8~1,1,9",
    ]
    .join("\n");
    assert_eq!(part1(&example), 5);
    assert_eq!(part2(&example), 7);
}
