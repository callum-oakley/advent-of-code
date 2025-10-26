use std::{cmp::Ordering, collections::HashMap, sync::LazyLock};

use regex::Regex;

#[derive(Clone, Copy)]
struct Range {
    min: u32,
    max: u32,
}

type Part<'a> = HashMap<&'a str, u32>;
type PartRange<'a> = HashMap<&'a str, Range>;

#[derive(Clone, Copy)]
struct Condition<'a> {
    stat: &'a str,
    ordering: Ordering,
    threshold: u32,
}

struct Workflow<'a> {
    steps: Vec<(Option<Condition<'a>>, &'a str)>,
}

impl<'a> Workflow<'a> {
    fn new(s: &'a str) -> Self {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"(\w+)(<|>)(\d+):(\w+)").unwrap());
        Self {
            steps: s
                .split(',')
                .map(|step| {
                    if let Some(captures) = RE.captures(step) {
                        (
                            Some(Condition {
                                stat: captures.get(1).unwrap().as_str(),
                                ordering: match &captures[2] {
                                    "<" => Ordering::Less,
                                    ">" => Ordering::Greater,
                                    _ => unreachable!(),
                                },
                                threshold: captures[3].parse().unwrap(),
                            }),
                            captures.get(4).unwrap().as_str(),
                        )
                    } else {
                        (None, step)
                    }
                })
                .collect(),
        }
    }

    fn process(&self, part: &Part) -> &'a str {
        for &(condition, dest) in &self.steps {
            if let Some(condition) = condition {
                if part[condition.stat].cmp(&condition.threshold) == condition.ordering {
                    return dest;
                }
            } else {
                return dest;
            }
        }
        unreachable!()
    }

    fn process_range(&self, part_range: &PartRange<'a>) -> Vec<(&'a str, PartRange<'_>)> {
        let mut res = Vec::new();
        let mut part_range = part_range.clone();
        for &(condition, dest) in &self.steps {
            if let Some(condition) = condition {
                match condition.ordering {
                    Ordering::Less => {
                        if part_range[condition.stat].max < condition.threshold {
                            res.push((dest, part_range));
                            break;
                        }
                        if part_range[condition.stat].min < condition.threshold {
                            let mut matching_part = part_range.clone();
                            matching_part.get_mut(condition.stat).unwrap().max =
                                condition.threshold - 1;
                            res.push((dest, matching_part));
                            part_range.get_mut(condition.stat).unwrap().min = condition.threshold;
                        }
                    }
                    Ordering::Greater => {
                        if part_range[condition.stat].min > condition.threshold {
                            res.push((dest, part_range));
                            break;
                        }
                        if part_range[condition.stat].max > condition.threshold {
                            let mut matching_part = part_range.clone();
                            matching_part.get_mut(condition.stat).unwrap().min =
                                condition.threshold + 1;
                            res.push((dest, matching_part));
                            part_range.get_mut(condition.stat).unwrap().max = condition.threshold;
                        }
                    }
                    Ordering::Equal => unreachable!(),
                }
            } else {
                res.push((dest, part_range));
                break;
            }
        }
        res
    }
}

fn parse(input: &str) -> (HashMap<&str, Workflow<'_>>, Vec<Part<'_>>) {
    let re = Regex::new(r"(\w*)\{([^{}]*)\}").unwrap();
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    (
        re.captures_iter(workflows)
            .map(|captures| {
                (
                    captures.get(1).unwrap().as_str(),
                    Workflow::new(captures.get(2).unwrap().as_str()),
                )
            })
            .collect(),
        re.captures_iter(parts)
            .map(|captures| {
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|stat| {
                        let (k, v) = stat.split_once('=').unwrap();
                        (k, v.parse().unwrap())
                    })
                    .collect()
            })
            .collect(),
    )
}

pub fn part1(input: &str) -> u32 {
    let (workflows, parts) = parse(input);
    parts
        .into_iter()
        .filter(|part| {
            let mut i = "in";
            loop {
                match i {
                    "A" => return true,
                    "R" => return false,
                    _ => i = workflows[i].process(part),
                }
            }
        })
        .flat_map(HashMap::into_values)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let (workflows, _) = parse(input);
    let mut res = 0;
    let mut q = vec![(
        "in",
        ["x", "m", "a", "s"]
            .into_iter()
            .map(|stat| (stat, Range { min: 1, max: 4000 }))
            .collect::<HashMap<_, _>>(),
    )];
    while let Some((i, part_range)) = q.pop() {
        match i {
            "A" => {
                res += part_range
                    .values()
                    .map(|range| u64::from(range.max - range.min + 1))
                    .product::<u64>();
            }
            "R" => {}
            _ => {
                for state in workflows[i].process_range(&part_range) {
                    q.push(state);
                }
            }
        }
    }
    res
}

pub fn tests() {
    let example = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";
    assert_eq!(part1(example), 19114);
    assert_eq!(part2(example), 167_409_079_868_000);
}
