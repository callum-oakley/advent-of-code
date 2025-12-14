use crate::grid::Vector;

fn parse(input: &str) -> (Vec<i64>, Vec<(Vector, Vec<i64>)>) {
    let mut paragraphs = input.trim().split("\n\n");
    (
        (&mut paragraphs)
            .take(6)
            .map(|p| i64::try_from(p.chars().filter(|&c| c == '#').count()).unwrap())
            .collect(),
        paragraphs
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let (region_size, presents) = line.trim().split_once(": ").unwrap();
                (
                    crate::cast::str_to_vector(region_size),
                    crate::cast::str_to_ints(presents).collect(),
                )
            })
            .collect(),
    )
}

fn possible(present_areas: &[i64], region_size: Vector, presents: &[i64]) -> bool {
    // First check if we even have enough area to fit every present in any configuration, if we
    // don't then we can return false immediately.
    let total_area: i64 = present_areas.iter().zip(presents).map(|(a, p)| a * p).sum();
    if total_area > region_size.x * region_size.y {
        return false;
    }

    // Then check if we have enough area to fit every present in a 3x3 square of its own, if we do
    // then we can return true immediately.
    if region_size.map(|a| a / 3).product() >= presents.iter().sum() {
        return true;
    }

    // Otherwise... this is a really hard problem[0], so hope we don't reach here!
    // [0] https://www.isnphard.com/i/polyomino-packing/
    unreachable!()
}

pub fn part1(input: &str) -> usize {
    let (present_areas, regions) = parse(input);
    regions
        .iter()
        .filter(|&(region_size, presents)| possible(&present_areas, *region_size, presents))
        .count()
}
