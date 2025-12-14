fn fuel1(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel2(mass: i32) -> i32 {
    let fuel = fuel1(mass);
    if fuel <= 0 { 0 } else { fuel + fuel2(fuel) }
}

pub fn part1(input: &str) -> i32 {
    input.lines().map(|s| fuel1(s.parse().unwrap())).sum()
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(|s| fuel2(s.parse().unwrap())).sum()
}

pub fn tests() {
    assert_eq!(fuel1(12), 2);
    assert_eq!(fuel1(14), 2);
    assert_eq!(fuel1(1969), 654);
    assert_eq!(fuel1(100_756), 33583);

    assert_eq!(fuel2(14), 2);
    assert_eq!(fuel2(1969), 966);
    assert_eq!(fuel2(100_756), 50346);
}
