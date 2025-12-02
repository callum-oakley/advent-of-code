use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use advent_of_code::{get_answer, get_input, solutions};

fn sig_figs(n: u32, duration: Duration) -> Duration {
    let nanos = u64::try_from(duration.as_nanos()).unwrap();
    if nanos == 0 {
        return duration;
    }
    let magnitude = 10u64.pow(nanos.ilog10() - n + 1);
    Duration::from_nanos(nanos / magnitude * magnitude)
}

fn run_part(year: u16, day: u8, part: u8, f: fn(&str) -> String, input: &str) -> (Duration, usize) {
    let now = Instant::now();
    let answer = f(input);
    let elapsed = now.elapsed();
    let mut stars = 0;

    let expected = get_answer(year, day, part);
    if let Some(ref expected) = expected {
        assert_eq!(&answer, expected);
        stars += 1;
    }

    println!(
        "{} {:0>2} {}   {: >5?}   {}{}",
        year,
        day,
        part,
        sig_figs(2, elapsed),
        answer,
        if stars == 0 { "   ?" } else { "" },
    );

    (elapsed, stars)
}

fn run_day(year: u16, day: u8, solution: &solutions::Solution) -> (Duration, usize) {
    if let Some(tests) = solution.tests {
        tests();
    }

    let input = get_input(year, day);
    let input = input.trim_end_matches('\n');

    let mut elapsed = Duration::new(0, 0);
    let mut stars = 0;
    if let Some(part1) = solution.part1 {
        let (e, s) = run_part(year, day, 1, part1, input);
        elapsed += e;
        stars += s;
    }
    if let Some(part2) = solution.part2 {
        let (e, s) = run_part(year, day, 2, part2, input);
        elapsed += e;
        stars += s;
    }
    (elapsed, stars)
}

fn run_year(year: u16, year_solutions: &BTreeMap<u8, solutions::Solution>) -> (Duration, usize) {
    let mut elapsed = Duration::new(0, 0);
    let mut stars = 0;
    for (day, solution) in year_solutions {
        let (e, s) = run_day(year, *day, solution);
        elapsed += e;
        stars += s;
    }

    if stars == 49 {
        // day 25 part 2
        stars += 1;
    }

    println!("{:\u{2500}^80}", "");
    println!(
        "{} ** *   {: >5?}   {} / {}",
        year,
        sig_figs(2, elapsed),
        stars,
        if year <= 2024 { 50 } else { 24 }
    );

    (elapsed, stars)
}

fn run(solutions: &BTreeMap<u16, BTreeMap<u8, solutions::Solution>>) {
    println!("{:\u{2500}^80}", "");

    let mut elapsed = Duration::new(0, 0);
    let mut stars = 0;
    let mut available = 0;
    for (&year, year_solutions) in solutions {
        let (e, s) = run_year(year, year_solutions);
        elapsed += e;
        stars += s;
        available += if year <= 2024 { 50 } else { 24 };
        println!("{:\u{2550}^80}", "");
    }

    println!(
        "**** ** *   {: >5?}   {} / {}",
        sig_figs(2, elapsed),
        stars,
        available,
    );
    println!("{:\u{2500}^80}", "");
}

fn main() {
    run(&solutions::build());
}
