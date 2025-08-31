use nalgebra::vector;

use crate::grid::Vector;

#[derive(Clone, Copy)]
struct Machine {
    a: Vector,
    b: Vector,
    prize: Vector,
}

fn parse(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.trim().split("\n\n").map(|s| {
        let mut vectors = s.lines().map(crate::cast::string_to_vector);
        Machine {
            a: vectors.next().unwrap(),
            b: vectors.next().unwrap(),
            prize: vectors.next().unwrap(),
        }
    })
}

fn play(Machine { a, b, prize }: Machine) -> Option<i64> {
    // The conditions to win a prize form a system of two simultaneous linear equations in two
    // unknowns, so with a bit of algebra...
    let a_top = b.x * prize.y - b.y * prize.x;
    let b_top = a.x * prize.y - a.y * prize.x;
    let a_bot = b.x * a.y - b.y * a.x;
    let b_bot = a.x * b.y - a.y * b.x;

    if a_top % a_bot == 0 && b_top % b_bot == 0 {
        Some(3 * a_top / a_bot + b_top / b_bot)
    } else {
        None
    }
}

pub fn part1(input: &str) -> i64 {
    parse(input).filter_map(play).sum()
}

pub fn part2(input: &str) -> i64 {
    parse(input)
        .filter_map(|machine| {
            play(Machine {
                prize: machine.prize + vector![10_000_000_000_000, 10_000_000_000_000],
                ..machine
            })
        })
        .sum()
}

pub fn tests() {
    let example = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    ";
    assert_eq!(part1(example), 480);
}
