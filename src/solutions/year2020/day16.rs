use std::collections::{HashMap, HashSet};

type Rules<'a> = HashMap<&'a str, Vec<(i64, i64)>>;
type Ticket = Vec<i64>;

fn parse_rules(input: &str) -> Rules<'_> {
    input
        .lines()
        .map(|line| {
            let (field, ranges) = line.split_once(':').unwrap();
            (
                field.trim(),
                ranges
                    .split("or")
                    .map(|range| {
                        let (low, high) = range.split_once('-').unwrap();
                        (low.trim().parse().unwrap(), high.trim().parse().unwrap())
                    })
                    .collect(),
            )
        })
        .collect()
}

fn parse_ticket(input: &str) -> Ticket {
    input
        .split(',')
        .map(|v| v.trim().parse().unwrap())
        .collect()
}

fn parse(input: &str) -> (Rules<'_>, Ticket, Vec<Ticket>) {
    let mut sections = input.trim().split("\n\n");
    (
        parse_rules(sections.next().unwrap()),
        parse_ticket(sections.next().unwrap().lines().nth(1).unwrap()),
        sections
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(parse_ticket)
            .collect(),
    )
}

fn valid(rules: &Rules, field: &str, v: i64) -> bool {
    rules[field]
        .iter()
        .any(|&(low, high)| low <= v && v <= high)
}

fn any_valid(rules: &Rules, v: i64) -> bool {
    rules.keys().any(|field| valid(rules, field, v))
}

fn part2_(input: &str) -> HashMap<&str, i64> {
    let (rules, ticket, mut nearby) = parse(input);
    nearby.retain(|ticket| ticket.iter().all(|&v| any_valid(&rules, v)));

    let mut field_options: HashMap<&str, HashSet<usize>> = HashMap::new();
    for i in 0..ticket.len() {
        for &field in rules.keys() {
            if nearby.iter().all(|ticket| valid(&rules, field, ticket[i])) {
                field_options.entry(field).or_default().insert(i);
            }
        }
    }

    let mut res = HashMap::new();
    while let Some((&field, options)) = field_options.iter().find(|(_, options)| options.len() == 1)
    {
        let i = *options.iter().next().unwrap();
        res.insert(field, ticket[i]);
        field_options.remove(field);
        for options in field_options.values_mut() {
            options.remove(&i);
        }
    }

    res
}

pub fn part1(input: &str) -> i64 {
    let (rules, _, nearby) = parse(input);
    nearby
        .into_iter()
        .flatten()
        .filter(|&v| !any_valid(&rules, v))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    part2_(input)
        .into_iter()
        .filter(|(field, _)| field.starts_with("departure"))
        .map(|(_, value)| value)
        .product()
}

pub fn tests() {
    let example1 = "
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    ";
    let example2 = "
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9
    ";
    assert_eq!(part1(example1), 71);
    assert_eq!(
        part2_(example2),
        HashMap::from([("class", 12), ("row", 11), ("seat", 13)]),
    );
}
