use std::collections::{HashMap, HashSet};

use regex::Regex;

struct Prog<'a> {
    weight: u32,
    children: Vec<&'a str>,
}

fn parse(input: &str) -> HashMap<&str, Prog<'_>> {
    let name_re = Regex::new(r"[a-z]+").unwrap();
    let weight_re = Regex::new(r"[0-9]+").unwrap();
    input
        .lines()
        .map(|line| {
            let mut names = name_re.find_iter(line).map(|m| m.as_str());
            let weight = weight_re.find(line).unwrap().as_str().parse().unwrap();
            (
                names.next().unwrap(),
                Prog {
                    weight,
                    children: names.collect(),
                },
            )
        })
        .collect()
}

fn part1_<'a>(tree: &HashMap<&'a str, Prog<'a>>) -> &'a str {
    let children = tree
        .values()
        .flat_map(|prog| prog.children.iter().copied())
        .collect::<HashSet<_>>();
    tree.keys().find(|&&name| !children.contains(name)).unwrap()
}

pub fn part1(input: &str) -> &str {
    part1_(&parse(input))
}

fn total_weight(tree: &HashMap<&str, Prog>, name: &str) -> u32 {
    tree[name].weight
        + tree[name]
            .children
            .iter()
            .map(|child| total_weight(tree, child))
            .sum::<u32>()
}

fn is_balanced(tree: &HashMap<&str, Prog>, name: &str) -> bool {
    let mut child_weights = tree[name]
        .children
        .iter()
        .map(|child| total_weight(tree, child));
    if let Some(w) = child_weights.next() {
        child_weights.all(|x| x == w)
    } else {
        true
    }
}

fn odd_one_out<'a>(tree: &HashMap<&'a str, Prog<'a>>, name: &'a str) -> &'a str {
    let mut children_by_weight = HashMap::new();
    for &child in &tree[name].children {
        children_by_weight
            .entry(total_weight(tree, child))
            .or_insert_with(Vec::new)
            .push(child);
    }
    children_by_weight
        .values()
        .find(|children| children.len() == 1)
        .unwrap()[0]
}

pub fn part2(input: &str) -> u32 {
    let tree = parse(input);
    let mut name = part1_(&tree);
    while let Some(unbalanced) = tree[name]
        .children
        .iter()
        .find(|child| !is_balanced(&tree, child))
    {
        name = unbalanced;
    }
    // name now refers to the deepest unbalanced program, so the program with
    // the wrong weight must be one of the children of this program.
    let wrong = odd_one_out(&tree, name);
    let right = tree[name]
        .children
        .iter()
        .find(|&&child| child != wrong)
        .unwrap();
    tree[wrong].weight + total_weight(&tree, right) - total_weight(&tree, wrong)
}

pub fn tests() {
    let example = "pbga (66)
                   xhth (57)
                   ebii (61)
                   havc (66)
                   ktlj (57)
                   fwft (72) -> ktlj, cntj, xhth
                   qoyq (66)
                   padx (45) -> pbga, havc, qoyq
                   tknk (41) -> ugml, padx, fwft
                   jptl (61)
                   ugml (68) -> gyxo, ebii, jptl
                   gyxo (61)
                   cntj (57)";
    assert_eq!(part1(example), "tknk");
    assert_eq!(part2(example), 60);
}
