use regex::Regex;

fn parse(input: &str) -> (u64, u64) {
    let re = Regex::new(r"\d+").unwrap();
    let mut matches = re.find_iter(input);
    (
        matches.next().unwrap().as_str().parse().unwrap(),
        matches.next().unwrap().as_str().parse().unwrap(),
    )
}

fn mod_pow(a: u64, b: u64, m: u64) -> u64 {
    if b == 0 {
        1
    } else if b.is_multiple_of(2) {
        mod_pow(a * a % m, b / 2, m)
    } else {
        a * mod_pow(a * a % m, b / 2, m) % m
    }
}

fn code(n: u64) -> u64 {
    20_151_125 * mod_pow(252_533, n - 1, 33_554_393) % 33_554_393
}

fn triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn part1(input: &str) -> u64 {
    let (row, col) = parse(input);
    code(col + triangle(row + col - 2))
}
