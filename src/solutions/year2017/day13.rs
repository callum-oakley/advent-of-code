struct Scanner {
    depth: u32,
    range: u32,
}

impl Scanner {
    fn caught(&self, delay: u32) -> bool {
        (delay + self.depth).is_multiple_of(2 * (self.range - 1))
    }
}

fn parse(input: &str) -> impl Iterator<Item = Scanner> + '_ {
    input.lines().map(|line| {
        let mut words = line.split(": ");
        Scanner {
            depth: words.next().unwrap().parse().unwrap(),
            range: words.next().unwrap().parse().unwrap(),
        }
    })
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .filter(|scanner| scanner.caught(0))
        .map(|scanner| scanner.depth * scanner.range)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let scanners: Vec<_> = parse(input).collect();
    for delay in 0.. {
        if scanners.iter().all(|scanner| !scanner.caught(delay)) {
            return delay;
        }
    }
    unreachable!()
}

pub fn tests() {
    let example = "0: 3\n1: 2\n4: 4\n6: 4";
    assert_eq!(part1(example), 24);
    assert_eq!(part2(example), 10);
}
