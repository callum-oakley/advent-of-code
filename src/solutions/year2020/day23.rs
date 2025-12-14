fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap() - 1)
        .collect()
}

fn play(cups: Vec<usize>, moves: usize) -> Vec<usize> {
    fn wrapping_pred(n: usize, max: usize) -> usize {
        if n == 0 { max } else { n - 1 }
    }

    let mut next = vec![0; cups.len()];
    for i in 0..cups.len() {
        next[cups[i]] = cups[(i + 1) % cups.len()];
    }

    let mut current = cups[0];
    let max = cups.into_iter().max().unwrap();

    for _ in 0..moves {
        // Pick up three cups immediately clockwise of current
        let a = next[current];
        let b = next[a];
        let c = next[b];
        next[current] = next[c];

        let mut destination = wrapping_pred(current, max);
        while [a, b, c].contains(&destination) {
            destination = wrapping_pred(destination, max);
        }

        // Place them immediately clockwise of destination
        next[c] = next[destination];
        next[destination] = a;

        current = next[current];
    }

    next
}

pub fn part1(input: &str) -> String {
    let next = play(parse(input), 100);

    let mut res = String::new();
    let mut current = 0;
    for _ in 0..8 {
        current = next[current];
        res.push(char::from_digit(u32::try_from(current + 1).unwrap(), 10).unwrap());
    }
    res
}

pub fn part2(input: &str) -> usize {
    let mut cups = parse(input);
    cups.extend(9..1_000_000);
    let next = play(cups, 10_000_000);

    let a = next[0];
    let b = next[a];

    (a + 1) * (b + 1)
}

pub fn tests() {
    assert_eq!(part1("389125467"), "67384529");
    assert_eq!(part2("389125467"), 149_245_887_792);
}
