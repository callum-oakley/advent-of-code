fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|range| {
        let (low, high) = range.split_once('-').unwrap();
        (low.parse().unwrap(), high.parse().unwrap())
    })
}

fn repeats(id: &str, chunk_len: usize) -> bool {
    let mut chunks = id.as_bytes().chunks(chunk_len);
    let first_chunk = chunks.next().unwrap();
    chunks.all(|chunk| chunk == first_chunk)
}

fn part_<F: Fn(&str) -> bool>(input: &str, invalid: F) -> u64 {
    parse(input)
        .flat_map(|(low, high)| low..=high)
        .filter(|&id| invalid(&id.to_string()))
        .sum()
}

pub fn part1(input: &str) -> u64 {
    part_(input, |id| id.len() % 2 == 0 && repeats(id, id.len() / 2))
}

pub fn part2(input: &str) -> u64 {
    part_(input, |id| (1..id.len()).any(|i| repeats(id, i)))
}

pub fn tests() {
    let example = concat!(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
        "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
        "824824821-824824827,2121212118-2121212124",
    );
    assert_eq!(part1(example), 1_227_775_554);
    assert_eq!(part2(example), 4_174_379_265);
}
