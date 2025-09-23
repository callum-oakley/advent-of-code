use std::fmt::Write;

fn look_and_say(s: &str) -> String {
    let mut res = String::new();
    for chunk in s.as_bytes().chunk_by(|a, b| a == b) {
        write!(res, "{}{}", chunk.len(), char::from(chunk[0])).unwrap();
    }
    res
}

fn part_(n: usize, input: &str) -> usize {
    let mut res = input.to_owned();
    for _ in 0..n {
        res = look_and_say(&res);
    }
    res.len()
}

pub fn part1(input: &str) -> usize {
    part_(40, input)
}

pub fn part2(input: &str) -> usize {
    part_(50, input)
}

pub fn tests() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
}
