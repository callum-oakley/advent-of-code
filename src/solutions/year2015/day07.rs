use std::collections::HashMap;

use regex::Regex;

type Wire<'a> = &'a str;
type Signal = u16;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Input<'a> {
    Wire(Wire<'a>),
    Signal(Signal),
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(s: &'a str) -> Self {
        match s.parse() {
            Ok(signal) => Input::Signal(signal),
            _ => Input::Wire(s),
        }
    }
}

enum Gate<'a> {
    Noop(Input<'a>),
    Not(Input<'a>),
    And(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
    LShift(Input<'a>, Input<'a>),
    RShift(Input<'a>, Input<'a>),
}

type Circuit<'a> = HashMap<Wire<'a>, Gate<'a>>;

fn parse(input: &str) -> Circuit<'_> {
    let noop = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
    let not = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
    let binop = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)$").unwrap();

    let mut circuit = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if let Some(captures) = noop.captures(line) {
            circuit.insert(
                captures.get(2).unwrap().as_str(),
                Gate::Noop(captures.get(1).unwrap().as_str().into()),
            );
        } else if let Some(captures) = not.captures(line) {
            circuit.insert(
                captures.get(2).unwrap().as_str(),
                Gate::Not(captures.get(1).unwrap().as_str().into()),
            );
        } else if let Some(captures) = binop.captures(line) {
            circuit.insert(
                captures.get(4).unwrap().as_str(),
                match &captures[2] {
                    "AND" => Gate::And,
                    "OR" => Gate::Or,
                    "LSHIFT" => Gate::LShift,
                    "RSHIFT" => Gate::RShift,
                    _ => unreachable!(),
                }(
                    captures.get(1).unwrap().as_str().into(),
                    captures.get(3).unwrap().as_str().into(),
                ),
            );
        } else {
            panic!("failed to parse: {line}");
        }
    }
    circuit
}

fn emulate<'a>(
    cache: &mut HashMap<Wire<'a>, Signal>,
    circuit: &'a Circuit,
    input: Input<'a>,
) -> Signal {
    match input {
        Input::Signal(signal) => signal,
        Input::Wire(wire) => {
            if cache.contains_key(&wire) {
                cache[&wire]
            } else {
                let signal = match circuit[wire] {
                    Gate::Noop(a) => emulate(cache, circuit, a),
                    Gate::Not(a) => !emulate(cache, circuit, a),
                    Gate::And(a, b) => emulate(cache, circuit, a) & emulate(cache, circuit, b),
                    Gate::Or(a, b) => emulate(cache, circuit, a) | emulate(cache, circuit, b),
                    Gate::LShift(a, b) => emulate(cache, circuit, a) << emulate(cache, circuit, b),
                    Gate::RShift(a, b) => emulate(cache, circuit, a) >> emulate(cache, circuit, b),
                };
                cache.insert(wire, signal);
                signal
            }
        }
    }
}

pub fn part1(input: &str) -> Signal {
    emulate(&mut HashMap::new(), &parse(input), Input::Wire("a"))
}

pub fn part2(input: &str) -> Signal {
    let mut circuit = parse(input);
    circuit.insert("b", Gate::Noop(Input::Signal(part1(input))));
    emulate(&mut HashMap::new(), &circuit, Input::Wire("a"))
}

pub fn tests() {
    let circuit = parse(
        "123 -> x
         456 -> y
         x AND y -> d
         x OR y -> e
         x LSHIFT 2 -> f
         y RSHIFT 2 -> g
         NOT x -> h
         NOT y -> i",
    );
    let mut cache = HashMap::new();
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("d")), 72);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("e")), 507);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("f")), 492);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("g")), 114);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("h")), 65412);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("i")), 65079);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("x")), 123);
    assert_eq!(emulate(&mut cache, &circuit, Input::Wire("y")), 456);
}
