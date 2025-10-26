pub fn part1(input: &str) -> usize {
    let mut product = Vec::new();
    for &c in input.as_bytes() {
        match product.last() {
            Some(&prev) if c.abs_diff(prev) == 32 => {
                product.pop();
            }
            _ => {
                product.push(c);
            }
        }
    }
    product.len()
}

pub fn part2(input: &str) -> usize {
    ('a'..='z')
        .map(|unit| {
            part1(
                &input
                    .chars()
                    .filter(|c| c.to_ascii_lowercase() != unit)
                    .collect::<String>(),
            )
        })
        .min()
        .unwrap()
}

pub fn tests() {
    assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
}
