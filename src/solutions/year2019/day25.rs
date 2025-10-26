use std::io::{BufReader, Read, Write};

use anyhow::{Context, Result};
use regex::Regex;

use crate::intcode::{State, VM};

pub fn play(prog: &str, r: impl Read, mut w: impl Write) -> Result<()> {
    let mut vm = VM::new(prog);
    let mut input = BufReader::new(r).bytes();
    loop {
        match vm.state() {
            State::Input => {
                vm.input(input.next().context("EOF")??.into());
            }
            State::Output => {
                write!(&mut w, "{}", char::from(u8::try_from(vm.output())?))?;
            }
            State::Halt => {
                return Ok(());
            }
        }
    }
}

pub fn part1(prog: &str) -> u32 {
    let mut input = [
        "west",
        "west",
        "north",
        "take space heater",
        "south",
        "east",
        "south",
        "south",
        "take sand",
        "north",
        "north",
        "east",
        "east",
        "take mug",
        "east",
        "south",
        "east",
        "south",
        "take easter egg",
        "north",
        "west",
        "west",
        "south",
        "west",
        "south",
        "south",
    ]
    .join("\n");
    input.push('\n');
    let mut output = Vec::new();

    play(prog, input.as_bytes(), &mut output).unwrap();

    Regex::new(r"You should be able to get in by typing (\d+) on the keypad at the main airlock.")
        .unwrap()
        .captures(&String::from_utf8(output).unwrap())
        .unwrap()[1]
        .parse()
        .unwrap()
}
