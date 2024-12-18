use std::collections::HashMap;

use crate::search;

struct Reaction<'a> {
    quantity: i64,
    reactants: Vec<(i64, &'a str)>,
}

fn parse(input: &str) -> HashMap<&str, Reaction> {
    fn parse_quantified(s: &str) -> (i64, &str) {
        let (quantity, chemical) = s.split_once(' ').unwrap();
        (quantity.parse().unwrap(), chemical)
    }
    input
        .lines()
        .map(|line| {
            let (reactants, product) = line.split_once(" => ").unwrap();
            let (quantity, product) = parse_quantified(product);
            (
                product,
                Reaction {
                    quantity,
                    reactants: reactants.split(", ").map(parse_quantified).collect(),
                },
            )
        })
        .collect()
}

// TODO remove when i64::div_ceil is stabilised
// https://doc.rust-lang.org/std/primitive.i64.html#method.div_ceil
fn div_ceil(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}

// Calculate the cost by performaing reactions in reverse, converting an amount of product in to the
// smallest amount of reactants required to produce that much product, until we only have ORE left.
// We can have negative quantities of chemicals during this process, which represents surplus
// product generated in a previous reaction, which we can use next time we need that chemical.
fn cost(reactions: &HashMap<&str, Reaction>, fuel: i64) -> i64 {
    let mut chemicals = HashMap::from([("FUEL", fuel)]);
    while let Some((chemical, quantity)) = chemicals
        .iter_mut()
        .find(|(&chemical, &mut quantity)| chemical != "ORE" && quantity > 0)
    {
        let k = div_ceil(*quantity, reactions[chemical].quantity);
        *quantity -= k * reactions[chemical].quantity;
        for (q, r) in &reactions[chemical].reactants {
            *chemicals.entry(r).or_default() += k * q;
        }
    }
    chemicals["ORE"]
}

pub fn part1(input: &str) -> i64 {
    cost(&parse(input), 1)
}

pub fn part2(input: &str) -> i64 {
    let reactions = parse(input);
    search::exponential(1, |i| cost(&reactions, i) > 1_000_000_000_000) - 1
}

pub fn tests() {
    let example0 = &[
        "10 ORE => 10 A",
        "1 ORE => 1 B",
        "7 A, 1 B => 1 C",
        "7 A, 1 C => 1 D",
        "7 A, 1 D => 1 E",
        "7 A, 1 E => 1 FUEL",
    ]
    .join("\n");
    let example1 = &[
        "9 ORE => 2 A",
        "8 ORE => 3 B",
        "7 ORE => 5 C",
        "3 A, 4 B => 1 AB",
        "5 B, 7 C => 1 BC",
        "4 C, 1 A => 1 CA",
        "2 AB, 3 BC, 4 CA => 1 FUEL",
    ]
    .join("\n");
    let example2 = &[
        "157 ORE => 5 NZVS",
        "165 ORE => 6 DCFZ",
        "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
        "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
        "179 ORE => 7 PSHF",
        "177 ORE => 5 HKGWZ",
        "7 DCFZ, 7 PSHF => 2 XJWVT",
        "165 ORE => 2 GPVTF",
        "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    ]
    .join("\n");
    let example3 = &[
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
        "17 NVRVD, 3 JNWZP => 8 VPVL",
        "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
        "22 VJHF, 37 MNCFX => 5 FWMGM",
        "139 ORE => 4 NVRVD",
        "144 ORE => 7 JNWZP",
        "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
        "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
        "145 ORE => 6 MNCFX",
        "1 NVRVD => 8 CXFTF",
        "1 VJHF, 6 MNCFX => 4 RFSQX",
        "176 ORE => 6 VJHF",
    ]
    .join("\n");
    let example4 = &[
        "171 ORE => 8 CNZTR",
        "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
        "114 ORE => 4 BHXH",
        "14 VRPVC => 6 BMBT",
        "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
        "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
        "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
        "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
        "5 BMBT => 4 WPTQ",
        "189 ORE => 9 KTJDG",
        "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
        "12 VRPVC, 27 CNZTR => 2 XDBXC",
        "15 KTJDG, 12 BHXH => 5 XCVML",
        "3 BHXH, 2 VRPVC => 7 MZWV",
        "121 ORE => 7 VRPVC",
        "7 XCVML => 6 RJRHP",
        "5 BHXH, 4 VRPVC => 5 LTCX",
    ]
    .join("\n");

    assert_eq!(part1(example0), 31);
    assert_eq!(part1(example1), 165);
    assert_eq!(part1(example2), 13312);
    assert_eq!(part1(example3), 180_697);
    assert_eq!(part1(example4), 2_210_736);

    assert_eq!(part2(example2), 82_892_753);
    assert_eq!(part2(example3), 5_586_022);
    assert_eq!(part2(example4), 460_664);
}
