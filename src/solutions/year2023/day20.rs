use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
enum ModuleKind<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcast,
}

#[derive(Clone)]
struct Module<'a> {
    kind: ModuleKind<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn tick(&mut self, src: &'a str, signal: bool) -> Option<bool> {
        match &mut self.kind {
            ModuleKind::FlipFlop(state) => {
                if !signal {
                    *state = !*state;
                    return Some(*state);
                }
            }
            ModuleKind::Conjunction(state) => {
                state.insert(src, signal);
                return Some(state.values().any(|s| !s));
            }
            ModuleKind::Broadcast => {
                return Some(signal);
            }
        }
        None
    }
}

#[derive(Clone)]
struct Circuit<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl Circuit<'_> {
    fn tick(&mut self, mut hook: impl FnMut((&str, &str, bool))) {
        let mut q = VecDeque::from([("button", "broadcaster", false)]);
        while let Some((src, dst, signal)) = q.pop_front() {
            hook((src, dst, signal));
            if let Some(module) = self.modules.get_mut(dst)
                && let Some(output) = module.tick(src, signal)
            {
                for &d in &module.destinations {
                    q.push_back((dst, d, output));
                }
            }
        }
    }

    fn period(&self, target: (&str, &str, bool)) -> usize {
        let mut circuit = self.clone();
        for i in 1.. {
            let mut seen = false;
            circuit.tick(|pulse| {
                if pulse == target {
                    seen = true;
                }
            });
            if seen {
                return i;
            }
        }
        unreachable!()
    }
}

fn parse(input: &str) -> Circuit<'_> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let (name, kind) = match &left[..1] {
                "%" => (&left[1..], ModuleKind::FlipFlop(false)),
                "&" => (&left[1..], ModuleKind::Conjunction(HashMap::new())),
                _ if left == "broadcaster" => (left, ModuleKind::Broadcast),
                _ => unreachable!(),
            };
            let destinations = right.split(", ").collect();
            (name, Module { kind, destinations })
        })
        .collect::<HashMap<_, _>>();
    for name in modules.keys().copied().collect::<Vec<_>>() {
        for dst in modules[name].destinations.clone() {
            if let Some(Module {
                kind: ModuleKind::Conjunction(state),
                ..
            }) = modules.get_mut(dst)
            {
                state.insert(name, false);
            }
        }
    }
    Circuit { modules }
}

pub fn part1(input: &str) -> u32 {
    let mut circuit = parse(input);
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        circuit.tick(|(_, _, signal)| {
            if signal {
                high += 1;
            } else {
                low += 1;
            }
        });
    }
    low * high
}

// Inspecting the input: rx is the output of &bq. &bq's inputs are each counters that send a high
// signal every <large prime> button presses, so finding the period of each and taking the product
// gives us the full number of button presses we would need for them to all send a high signal
// simultaneously. Circuit diagram:
// https://excalidraw.com/#json=YXARkwgJg7R8bvOFZ370l,EeS99kyZkaxmg6E91aYYrw
pub fn part2(input: &str) -> usize {
    let circuit = parse(input);
    ["gc", "kp", "vg", "tx"]
        .into_iter()
        .map(|src| circuit.period((src, "bq", true)))
        .product()
}

pub fn tests() {
    let example1 = [
        "broadcaster -> a, b, c",
        "%a -> b",
        "%b -> c",
        "%c -> inv",
        "&inv -> a",
    ]
    .join("\n");
    let example2 = [
        "broadcaster -> a",
        "%a -> inv, con",
        "&inv -> b",
        "%b -> con",
        "&con -> output",
    ]
    .join("\n");
    assert_eq!(part1(&example1), 32_000_000);
    assert_eq!(part1(&example2), 11_687_500);
}
