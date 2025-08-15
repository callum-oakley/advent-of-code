use std::collections::{HashSet, VecDeque};

use regex::Regex;

use crate::grid::{Bounds, Vector, E, N, S, W};

fn parse(input: &str) -> HashSet<Vector> {
    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)").unwrap();
    re.captures_iter(input)
        .flat_map(|captures| {
            let fixed: i64 = captures[2].parse().unwrap();
            let start: i64 = captures[4].parse().unwrap();
            let end: i64 = captures[5].parse().unwrap();
            (start..=end).map(move |v| match (&captures[1], &captures[3]) {
                ("x", "y") => Vector::new(fixed, v),
                ("y", "x") => Vector::new(v, fixed),
                _ => unreachable!(),
            })
        })
        .collect()
}

fn flow(
    clay: &HashSet<Vector>,
    flowing: &HashSet<Vector>,
    settled: &HashSet<Vector>,
    block: Vector,
) -> Vec<Vector> {
    [block + W, block + E]
        .into_iter()
        .filter(|p| !clay.contains(p) && !flowing.contains(p) && !settled.contains(p))
        .collect()
}

fn settle(
    clay: &HashSet<Vector>,
    flowing: &HashSet<Vector>,
    settled: &HashSet<Vector>,
    block: Vector,
) -> Vec<Vector> {
    let settle_dir = |dir| {
        let mut res = Vec::new();
        let mut block = block;
        while flowing.contains(&block)
            && (clay.contains(&(block + S)) || settled.contains(&(block + S)))
        {
            block += dir;
            if clay.contains(&block) {
                return Some(res);
            }
            res.push(block);
        }
        None
    };

    let mut res = Vec::new();
    if let (Some(mut west), Some(mut east)) = (settle_dir(W), settle_dir(E)) {
        res.append(&mut west);
        res.push(block);
        res.append(&mut east);
    }
    res
}

fn part_(input: &str) -> (usize, usize) {
    let clay = parse(input);
    let bounds = Bounds::new(clay.iter().copied());

    let mut flowing = HashSet::from([Vector::new(500, 0)]);
    let mut settled = HashSet::new();
    let mut queue = VecDeque::from([Vector::new(500, 0)]);

    while let Some(block) = queue.pop_front() {
        let down = block + S;

        if flowing.contains(&down) || down.y > bounds.max.y {
            continue;
        }

        if clay.contains(&down) || settled.contains(&down) {
            let new_flowing = flow(&clay, &flowing, &settled, block);
            if !new_flowing.is_empty() {
                flowing.extend(new_flowing.iter().copied());
                queue.extend(new_flowing);
                continue;
            }

            let new_settled = settle(&clay, &flowing, &settled, block);
            for p in &new_settled {
                flowing.remove(p);
            }
            settled.extend(new_settled.iter().copied());
            queue.extend(
                new_settled
                    .into_iter()
                    .map(|p| p + N)
                    .filter(|p| flowing.contains(p)),
            );
            continue;
        }

        flowing.insert(down);
        queue.push_back(down);
    }

    (
        flowing
            .into_iter()
            .filter(|&p| p.y >= bounds.min.y && p.y <= bounds.max.y)
            .count(),
        settled
            .into_iter()
            .filter(|&p| p.y >= bounds.min.y && p.y <= bounds.max.y)
            .count(),
    )
}

pub fn part1(input: &str) -> usize {
    let (flowing, settled) = part_(input);
    flowing + settled
}

pub fn part2(input: &str) -> usize {
    part_(input).1
}

pub fn tests() {
    let example = "
        x=495, y=2..7
        y=7, x=495..501
        x=501, y=3..7
        x=498, y=2..4
        x=506, y=1..2
        x=498, y=10..13
        x=504, y=10..13
        y=13, x=498..504
    ";
    assert_eq!(part1(example), 57);
    assert_eq!(part2(example), 29);
}
