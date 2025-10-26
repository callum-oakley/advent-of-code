use std::collections::HashMap;

type Bag<'a> = HashMap<&'a str, u32>;

struct Game<'a> {
    id: usize,
    samples: Vec<Bag<'a>>,
}

fn parse(input: &str) -> impl Iterator<Item = Game<'_>> {
    input.lines().enumerate().map(|(i, line)| {
        let id = i + 1;
        let samples = line
            .strip_prefix(&format!("Game {id}: "))
            .unwrap()
            .split("; ")
            .map(|sample| {
                sample
                    .split(", ")
                    .map(|pair| {
                        let (n, colour) = pair.split_once(' ').unwrap();
                        (colour, n.parse().unwrap())
                    })
                    .collect()
            })
            .collect();
        Game { id, samples }
    })
}

fn merge_with_max<'a>(bags: impl IntoIterator<Item = &'a Bag<'a>>) -> Bag<'a> {
    let mut res: Bag = HashMap::new();
    for bag in bags {
        for (&colour, &n) in bag {
            res.entry(colour)
                .and_modify(|v| *v = (*v).max(n))
                .or_insert(n);
        }
    }
    res
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|game| {
            let min_bag = merge_with_max(&game.samples);
            min_bag["red"] <= 12 && min_bag["green"] <= 13 && min_bag["blue"] <= 14
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    parse(input)
        .map(|game| merge_with_max(&game.samples).values().product::<u32>())
        .sum()
}

pub fn tests() {
    let example = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ]
    .join("\n");

    assert_eq!(part1(&example), 8);
    assert_eq!(part2(&example), 2286);
}
