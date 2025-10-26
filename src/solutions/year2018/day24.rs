use std::{cmp::Reverse, sync::LazyLock};

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Army {
    ImmuneSystem,
    Infection,
}

#[derive(Clone)]
struct Group<'a> {
    army: Army,
    units: u32,
    hit_points: u32,
    attack_damage: u32,
    attack_type: &'a str,
    initiative: u32,
    weaknesses: Vec<&'a str>,
    immunities: Vec<&'a str>,
    attacking: Option<usize>,
    defending: bool,
}

fn parse_group(army: Army, s: &str) -> Group<'_> {
    static GROUP: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(\d+) units each with (\d+) hit points (\([^)]+\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)",
        ).unwrap()
    });
    static MODIFIERS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(weak|immune) to ([\w, ]+)").unwrap());

    let group = GROUP.captures(s).unwrap();

    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();
    if let Some(modifiers) = group.get(3) {
        for modifiers in MODIFIERS.captures_iter(modifiers.as_str()) {
            match &modifiers[1] {
                "weak" => weaknesses.extend(modifiers.get(2).unwrap().as_str().split(", ")),
                "immune" => immunities.extend(modifiers.get(2).unwrap().as_str().split(", ")),
                _ => unreachable!(),
            }
        }
    }

    Group {
        army,
        units: group[1].parse().unwrap(),
        hit_points: group[2].parse().unwrap(),
        attack_damage: group[4].parse().unwrap(),
        attack_type: group.get(5).unwrap().as_str(),
        initiative: group[6].parse().unwrap(),
        weaknesses,
        immunities,
        attacking: None,
        defending: false,
    }
}

fn parse(input: &str) -> Vec<Group<'_>> {
    let (immune_system, infection) = input.split_once("\n\n").unwrap();
    let mut groups: Vec<_> = immune_system
        .trim()
        .lines()
        .skip(1)
        .map(|line| parse_group(Army::ImmuneSystem, line))
        .chain(
            infection
                .trim()
                .lines()
                .skip(1)
                .map(|line| parse_group(Army::Infection, line)),
        )
        .collect();
    groups.sort_unstable_by_key(|group| Reverse(group.initiative));
    groups
}

impl Group<'_> {
    fn effective_power(&self) -> u32 {
        self.units * self.attack_damage
    }

    fn damage(&self, defender: &Self) -> u32 {
        if defender.immunities.contains(&self.attack_type) {
            0
        } else if defender.weaknesses.contains(&self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }
}

fn select_targets(groups: &mut [Group]) {
    let mut priority: Vec<_> = (0..groups.len()).collect();
    priority
        .sort_unstable_by_key(|&i| Reverse((groups[i].effective_power(), groups[i].initiative)));
    for attacker in priority {
        if let Some(defender) = (0..groups.len())
            .filter(|&defender| {
                groups[defender].units > 0
                    && groups[defender].army != groups[attacker].army
                    && !groups[defender].defending
                    && groups[attacker].damage(&groups[defender]) > 0
            })
            .max_by_key(|&defender| {
                (
                    groups[attacker].damage(&groups[defender]),
                    groups[defender].effective_power(),
                    groups[defender].initiative,
                )
            })
        {
            groups[attacker].attacking = Some(defender);
            groups[defender].defending = true;
        }
    }
}

fn attack(groups: &mut [Group]) {
    for attacker in 0..groups.len() {
        if let Some(defender) = groups[attacker].attacking {
            groups[defender].units = groups[defender].units.saturating_sub(
                groups[attacker].damage(&groups[defender]) / groups[defender].hit_points,
            );
            groups[attacker].attacking = None;
            groups[defender].defending = false;
        }
    }
}

fn collect_dead(groups: &mut Vec<Group>) {
    let mut i = 0;
    while i < groups.len() {
        if groups[i].units == 0 {
            groups.remove(i);
        } else {
            i += 1;
        }
    }
}

fn count_units(groups: &[Group], army: Army) -> u32 {
    groups
        .iter()
        .filter(|group| group.army == army)
        .map(|group| group.units)
        .sum()
}

fn get_summary(groups: &[Group]) -> (u32, u32) {
    (
        count_units(groups, Army::ImmuneSystem),
        count_units(groups, Army::Infection),
    )
}

fn fight(mut groups: Vec<Group>) -> (u32, u32) {
    let mut summary = get_summary(&groups);
    loop {
        select_targets(&mut groups);
        attack(&mut groups);
        collect_dead(&mut groups);

        // Checking for zeros isn't enough because we can get stuck in a stalemate
        let new_summary = get_summary(&groups);
        if new_summary == summary {
            return summary;
        }
        summary = new_summary;
    }
}

pub fn part1(input: &str) -> u32 {
    let (immune_system_units, infection_units) = fight(parse(input));
    immune_system_units + infection_units
}

pub fn part2(input: &str) -> u32 {
    let groups = parse(input);
    for boost in 0.. {
        let mut groups = groups.clone();
        for group in &mut groups {
            if group.army == Army::ImmuneSystem {
                group.attack_damage += boost;
            }
        }
        let (immune_system_units, infection_units) = fight(groups);
        if infection_units == 0 {
            return immune_system_units;
        }
    }
    unreachable!()
}

pub fn tests() {
    let example = "
        Immune System:
        17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
        989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

        Infection:
        801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
        4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
    ";
    assert_eq!(part1(example), 5216);
    assert_eq!(part2(example), 51);
}
