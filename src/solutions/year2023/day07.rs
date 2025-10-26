use std::collections::HashMap;

use crate::part::Part;

const CARD_LOOKUP_1: &[u8] = b"23456789TJQKA";
const CARD_LOOKUP_2: &[u8] = b"J23456789TQKA";
const JOKER: usize = 0;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Category {
    fn new(part: Part, cards: &[usize]) -> Self {
        let mut freqs: HashMap<usize, i32> = HashMap::new();
        let mut jokers = 0;
        for &c in cards {
            if part == Part::Two && c == JOKER {
                jokers += 1;
            } else {
                *freqs.entry(c).or_default() += 1;
            }
        }

        let mut freqs = freqs.values().copied().collect::<Vec<_>>();
        freqs.sort_unstable();

        // Setting the wildcard to match the most frequently occuring other card maximises our score
        if let Some(last) = freqs.last_mut() {
            *last += jokers;
        } else {
            freqs.push(5);
        }

        match freqs[..] {
            [1, 1, 1, 1, 1] => Category::HighCard,
            [1, 1, 1, 2] => Category::OnePair,
            [1, 2, 2] => Category::TwoPair,
            [1, 1, 3] => Category::ThreeOfAKind,
            [2, 3] => Category::FullHouse,
            [1, 4] => Category::FourOfAKind,
            [5] => Category::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: Category,
    // Represent cards by their index in CARD_LOOKUP_* so that they sort by strength.
    cards: Vec<usize>,
}

fn parse(part: Part, input: &str) -> Vec<(Hand, usize)> {
    let card_lookup = match part {
        Part::One => CARD_LOOKUP_1,
        Part::Two => CARD_LOOKUP_2,
    };
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards
                .as_bytes()
                .iter()
                .map(|&card| card_lookup.iter().position(|&c| c == card).unwrap())
                .collect::<Vec<_>>();
            let kind = Category::new(part, &cards);
            (Hand { kind, cards }, bid.parse().unwrap())
        })
        .collect()
}

fn part_(part: Part, input: &str) -> usize {
    let mut hands = parse(part, input);
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

pub fn part1(input: &str) -> usize {
    part_(Part::One, input)
}

pub fn part2(input: &str) -> usize {
    part_(Part::Two, input)
}

pub fn tests() {
    let example = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    assert_eq!(part1(example), 6440);
    assert_eq!(part2(example), 5905);
}
