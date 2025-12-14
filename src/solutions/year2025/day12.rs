fn parse(input: &str) -> (Vec<usize>, Vec<(usize, Vec<usize>)>) {
    let mut paragraphs = input.trim().split("\n\n");
    (
        (&mut paragraphs)
            .take(6)
            .map(|p| p.chars().filter(|&c| c == '#').count())
            .collect(),
        paragraphs
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let (area, presents) = line.trim().split_once(": ").unwrap();
                (
                    crate::cast::str_to_ints::<usize>(area).product(),
                    crate::cast::str_to_ints(presents).collect(),
                )
            })
            .collect(),
    )
}

// As a first approximation, filter out the regions that aren't big enough to possibly contain all
// the presents. This turns out to be enough!
pub fn part1(input: &str) -> usize {
    let (present_areas, regions) = parse(input);
    regions
        .iter()
        .filter(|&(size, presents)| {
            presents
                .iter()
                .zip(&present_areas)
                .map(|(p, a)| p * a)
                .sum::<usize>()
                <= *size
        })
        .count()
}
