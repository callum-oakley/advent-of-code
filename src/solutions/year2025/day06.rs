type Worksheet<'a> = Vec<(Vec<u64>, &'a str)>;

fn parse1(input: &str) -> Worksheet<'_> {
    let lines: Vec<_> = input.lines().collect();
    let mut cols: Vec<Vec<u64>> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        for (i, s) in line.split_whitespace().enumerate() {
            if i >= cols.len() {
                cols.push(Vec::new());
            }
            cols[i].push(s.parse().unwrap());
        }
    }
    cols.into_iter()
        .zip(lines[lines.len() - 1].split_whitespace())
        .collect()
}

fn parse2(input: &str) -> Worksheet<'_> {
    let lines: Vec<_> = input.lines().collect();
    let mut cols: Vec<Vec<u64>> = vec![Vec::new()];
    for i in 0..lines[0].len() {
        let s: String = lines[..lines.len() - 1]
            .iter()
            .map(|line| &line[i..=i])
            .collect();
        if s.trim().is_empty() {
            cols.push(Vec::new());
        } else {
            cols.last_mut().unwrap().push(s.trim().parse().unwrap());
        }
    }
    cols.into_iter()
        .zip(lines[lines.len() - 1].split_whitespace())
        .collect()
}

fn part_(input: &str, parse: fn(&str) -> Worksheet) -> u64 {
    parse(input)
        .into_iter()
        .map(|(col, op)| match op {
            "+" => col.iter().sum::<u64>(),
            "*" => col.iter().product::<u64>(),
            _ => unreachable!(),
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    part_(input, parse1)
}

pub fn part2(input: &str) -> u64 {
    part_(input, parse2)
}

pub fn tests() {
    let example = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n",
    );
    assert_eq!(part1(example), 4_277_556);
    assert_eq!(part2(example), 3_263_827);
}
