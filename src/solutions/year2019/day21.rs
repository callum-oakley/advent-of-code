use std::io::{self, BufReader, Read, Write};

use anyhow::{Context, Result};

use crate::intcode::{State, VM};

pub fn play(vm: &mut VM, r: impl Read, mut w: impl Write) -> Result<Option<i64>> {
    let mut input = BufReader::new(r).bytes();
    loop {
        match vm.state() {
            State::Input => {
                vm.input(input.next().context("EOF")??.into());
            }
            State::Output => {
                let output = vm.output();
                if let Ok(c) = u8::try_from(output) {
                    write!(&mut w, "{}", char::from(c))?;
                } else {
                    // if output won't fit in a u8, assume it's the final damage reading
                    vm.halt();
                    return Ok(Some(output));
                }
            }
            State::Halt => {
                return Ok(None);
            }
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let springscript = "OR A T\nAND B T\nAND C T\nNOT T T\nAND D T\nOR T J\nWALK\n";
    play(&mut VM::new(input), springscript.as_bytes(), io::empty())
        .unwrap()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let springscript = "OR A T\nAND B T\nAND C T\nNOT T T\nAND D T\nOR E J\nOR H J\nAND T J\nRUN\n";
    play(&mut VM::new(input), springscript.as_bytes(), io::empty())
        .unwrap()
        .unwrap()
}
