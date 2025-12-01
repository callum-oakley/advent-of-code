fn parse(input: &str) -> impl Iterator<Item = i32> {
    input.lines().map(|line| match &line[..1] {
        "L" => -line[1..].parse::<i32>().unwrap(),
        "R" => line[1..].parse::<i32>().unwrap(),
        _ => unreachable!(),
    })
}

fn part_(rotations: impl Iterator<Item = i32>) -> usize {
    rotations
        .scan(50, |dial, rotation| {
            *dial = (*dial + rotation) % 100;
            Some(*dial)
        })
        .filter(|&dial| dial == 0)
        .count()
}

pub fn part1(input: &str) -> usize {
    part_(parse(input))
}

pub fn part2(input: &str) -> usize {
    part_(parse(input).flat_map(|rotation| {
        std::iter::repeat_n(rotation.signum(), usize::try_from(rotation.abs()).unwrap())
    }))
}

pub fn tests() {
    let example = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    assert_eq!(part1(example), 3);
    assert_eq!(part2(example), 6);
}
