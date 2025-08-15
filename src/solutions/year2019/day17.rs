use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

use anyhow::{Context, Result};

use crate::{
    grid::{self, Adjacent, IntoVector, Vector, LEFT, RIGHT},
    intcode::{State, VM},
};

pub fn play(vm: &mut VM, r: impl Read, mut w: impl Write) -> Result<Option<i64>> {
    let mut input = r.bytes();
    loop {
        match vm.state() {
            State::Input => {
                vm.input(input.next().context("EOF")??.into());
            }
            State::Output => {
                let output = vm.output();
                if let Ok(c) = u8::try_from(output) {
                    write!(&mut w, "{}", char::from(c))?;
                } else {
                    // if output won't fit in a u8, assume it's the final dust reading
                    vm.halt();
                    return Ok(Some(output));
                }
            }
            State::Halt => {
                return Ok(None);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Robot {
    pos: Vector,
    dir: Vector,
}

fn parse(map: &str) -> (HashSet<Vector>, Robot) {
    let mut scaffold = HashSet::new();
    let mut robot = None;
    grid::scan(map).for_each(|(pos, c)| match c {
        '#' => {
            scaffold.insert(pos);
        }
        '^' | '>' | 'v' | '<' => {
            scaffold.insert(pos);
            robot = Some(Robot {
                pos,
                dir: c.into_vector(),
            });
        }
        '.' => {}
        _ => unreachable!(),
    });
    (scaffold, robot.unwrap())
}

fn part1_(map: &str) -> i64 {
    let (scaffold, _) = parse(map);
    scaffold
        .iter()
        .filter(|&&pos| pos.adjacent4().all(|p| scaffold.contains(&p)))
        .map(|pos| pos.y * pos.x)
        .sum()
}

type Path = Vec<(char, u8)>;
type PathRef<'a> = &'a [(char, u8)];

fn path(scaffold: &HashSet<Vector>, mut robot: Robot) -> Path {
    let mut res: Path = Vec::new();
    loop {
        if scaffold.contains(&(robot.pos + robot.dir)) {
            robot.pos += robot.dir;
            res.last_mut().unwrap().1 += 1;
        } else if scaffold.contains(&(robot.pos + LEFT * robot.dir)) {
            robot.dir = LEFT * robot.dir;
            robot.pos += robot.dir;
            res.push(('L', 1));
        } else if scaffold.contains(&(robot.pos + RIGHT * robot.dir)) {
            robot.dir = RIGHT * robot.dir;
            robot.pos += robot.dir;
            res.push(('R', 1));
        } else {
            break;
        }
    }
    res
}

fn path_to_string(path: PathRef) -> String {
    use std::fmt::Write;

    let mut res = String::new();
    for (c, n) in path {
        if !res.is_empty() {
            res.push(',');
        }
        write!(res, "{c},{n}").unwrap();
    }
    res
}

fn try_compress(mut path: PathRef, a: PathRef, b: PathRef, c: PathRef) -> Option<String> {
    let mut res = String::new();
    let mut push = |c| {
        if !res.is_empty() {
            res.push(',');
        }
        res.push(c);
    };
    while !path.is_empty() {
        if path.starts_with(a) {
            push('A');
            path = &path[a.len()..];
        } else if path.starts_with(b) {
            push('B');
            path = &path[b.len()..];
        } else if path.starts_with(c) {
            push('C');
            path = &path[c.len()..];
        } else {
            return None;
        }
    }
    Some(res)
}

fn compress(path: PathRef) -> String {
    const MAX_SUBROUTINE_LEN: usize = 5;
    fn trim_start<'a>(mut p: PathRef<'a>, subroutines: &[PathRef<'a>]) -> PathRef<'a> {
        while let Some(s) = subroutines.iter().find(|s| p.starts_with(s)) {
            p = &p[s.len()..];
        }
        p
    }
    for i in 1..=path.len().min(MAX_SUBROUTINE_LEN) {
        let a = &path[..i];
        let p = trim_start(path, &[a]);
        for j in 1..=p.len().min(MAX_SUBROUTINE_LEN) {
            let b = &p[..j];
            let p = trim_start(p, &[a, b]);
            for k in 1..=p.len().min(MAX_SUBROUTINE_LEN) {
                let c = &p[..k];
                if let Some(main) = try_compress(path, a, b, c) {
                    return format!(
                        "{}\n{}\n{}\n{}\n",
                        main,
                        path_to_string(a),
                        path_to_string(b),
                        path_to_string(c),
                    );
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> i64 {
    let mut buf = Vec::new();
    play(&mut VM::new(input), io::empty(), &mut buf).unwrap();
    part1_(String::from_utf8(buf).unwrap().trim())
}

pub fn part2(input: &str) -> i64 {
    let mut vm = VM::new(input);

    let mut buf = Vec::new();
    play(&mut vm.clone(), io::empty(), &mut buf).unwrap();
    let (scaffold, robot) = parse(String::from_utf8(buf).unwrap().trim());

    let path = path(&scaffold, robot);
    let mut instructions = compress(&path);
    instructions.push_str("n\n"); // Continuous video feed?

    vm.mem[0] = 2;
    play(&mut vm, instructions.as_bytes(), io::empty())
        .unwrap()
        .unwrap()
}

pub fn tests() {
    let example1 = [
        "..#..........",
        "..#..........",
        "#######...###",
        "#.#...#...#.#",
        "#############",
        "..#...#...#..",
        "..#####...^..",
    ]
    .join("\n");
    assert_eq!(part1_(&example1), 76);

    let example2 = [
        "#######...#####",
        "#.....#...#...#",
        "#.....#...#...#",
        "......#...#...#",
        "......#...###.#",
        "......#.....#.#",
        "^########...#.#",
        "......#.#...#.#",
        "......#########",
        "........#...#..",
        "....#########..",
        "....#...#......",
        "....#...#......",
        "....#...#......",
        "....#####......",
    ]
    .join("\n");
    let (scaffold, robot) = parse(&example2);
    let path = path(&scaffold, robot);
    assert_eq!(
        path_to_string(&path),
        "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
    );
}
