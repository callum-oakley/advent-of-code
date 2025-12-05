fn parse(input: &str) -> (Vec<(u64, u64)>, impl Iterator<Item = u64>) {
    let (fresh, ids) = input.split_once("\n\n").unwrap();
    (
        fresh
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect(),
        ids.lines().map(|line| line.parse().unwrap()),
    )
}

fn disjoint((a0, a1): (u64, u64), (b0, b1): (u64, u64)) -> bool {
    b1 < a0 || a1 < b0
}

fn union((a0, a1): (u64, u64), (b0, b1): (u64, u64)) -> (u64, u64) {
    (a0.min(b0), a1.max(b1))
}

pub fn part1(input: &str) -> usize {
    let (fresh, ids) = parse(input);
    ids.filter(|id| fresh.iter().any(|(a, b)| (a..=b).contains(&id)))
        .count()
}

pub fn part2(input: &str) -> u64 {
    let (fresh, _) = parse(input);
    let mut fresh_disjoint = Vec::new();
    for mut a in fresh {
        let mut fresh_disjoint_next = Vec::new();
        for b in fresh_disjoint {
            if disjoint(a, b) {
                fresh_disjoint_next.push(b);
            } else {
                a = union(a, b);
            }
        }
        fresh_disjoint_next.push(a);
        fresh_disjoint = fresh_disjoint_next;
    }
    fresh_disjoint.iter().map(|&(b0, b1)| b1 - b0 + 1).sum()
}

pub fn tests() {
    let example = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    assert_eq!(part1(example), 3);
    assert_eq!(part2(example), 14);
}
