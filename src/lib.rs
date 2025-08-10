#![cfg_attr(feature = "allow_dead_code", allow(dead_code))]
#![warn(clippy::pedantic)]
#![expect(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

use regex::Regex;
use std::{fs, path::Path};

pub mod intcode;
pub mod solutions;

mod combinatorics;
mod freqs;
mod grid;
mod hash;
mod lex_ord;
mod number_theory;
mod ocr;
mod part;
mod search;
mod sexp;
mod unbounded_vec;
mod uniq;
mod vm_2018;

fn get(path: &str) -> Result<String, String> {
    let res = match reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{path}"))
        .header(
            "cookie",
            format!("session={}", fs::read_to_string(".session").unwrap().trim()),
        )
        .send()
    {
        Ok(res) => res,
        Err(err) => {
            return Err(format!("failed to get {path}: {err}"));
        }
    };

    let status = res.status();

    let text = match res.text() {
        Ok(text) => text,
        Err(err) => {
            return Err(format!("failed to get {path}: {err}"));
        }
    };

    if status.is_client_error() || status.is_server_error() {
        return Err(format!(
            "failed to get {path}: unexpected status: {status}: {text}"
        ));
    }

    Ok(text)
}

pub fn get_input(year: u16, day: u8) -> String {
    let path = format!("input/{year}/{day:0>2}");
    let path = Path::new(&path);
    if path.exists() {
        fs::read_to_string(path).unwrap()
    } else {
        let input = get(&format!("{year}/day/{day}/input")).expect("failed to get input");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, &input).unwrap();
        input
    }
}

pub fn get_answer(year: u16, day: u8, part: u8) -> Option<String> {
    let path = format!("answer/{year}/{day:0>2}/{part}");
    let path = Path::new(&path);
    if path.exists() {
        Some(fs::read_to_string(path).unwrap())
    } else {
        let Ok(page) = get(&format!("{year}/day/{day}")) else {
            return None;
        };
        let mut answers: Vec<String> = Regex::new(r"Your puzzle answer was <code>([^<]*)")
            .unwrap()
            .captures_iter(&page)
            .map(|captures| captures[1].to_owned())
            .collect();
        for (i, answer) in answers.iter().enumerate() {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path.parent().unwrap().join((i + 1).to_string()), answer).unwrap();
        }
        if answers.len() >= part.into() {
            Some(answers.swap_remove(part as usize - 1))
        } else {
            None
        }
    }
}
