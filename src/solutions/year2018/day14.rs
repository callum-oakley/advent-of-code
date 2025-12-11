use crate::part::Part;

fn part_(part: Part, input: &str) -> String {
    let target1 = input.parse().unwrap();
    let target2 = input
        .chars()
        .map(|d| u8::try_from(d.to_digit(10).unwrap()).unwrap())
        .collect::<Vec<_>>();

    let check = |scores: &[u8]| match part {
        Part::One => {
            if scores.len() == target1 + 10 {
                Some(
                    scores[target1..target1 + 10]
                        .iter()
                        .map(|&d| char::from_digit(u32::from(d), 10).unwrap())
                        .collect(),
                )
            } else {
                None
            }
        }
        Part::Two => {
            if scores.len() >= target2.len() && scores[scores.len() - target2.len()..] == target2 {
                Some((scores.len() - target2.len()).to_string())
            } else {
                None
            }
        }
    };

    let mut scores: Vec<u8> = vec![3, 7];
    let mut elf0 = 0;
    let mut elf1 = 1;

    loop {
        let mix = scores[elf0] + scores[elf1];
        if mix < 10 {
            scores.push(mix);
        } else {
            scores.push(mix / 10);
            if let Some(res) = check(&scores) {
                return res;
            }
            scores.push(mix % 10);
        }
        if let Some(res) = check(&scores) {
            return res;
        }
        elf0 = (elf0 + 1 + usize::from(scores[elf0])) % scores.len();
        elf1 = (elf1 + 1 + usize::from(scores[elf1])) % scores.len();
    }
}

pub fn part1(input: &str) -> String {
    part_(Part::One, input).clone()
}

pub fn part2(input: &str) -> String {
    part_(Part::Two, input).clone()
}

pub fn tests() {
    assert_eq!(part1("9"), "5158916779");
    assert_eq!(part1("5"), "0124515891");
    assert_eq!(part1("18"), "9251071085");
    assert_eq!(part1("2018"), "5941429882");

    assert_eq!(part2("51589"), "9");
    assert_eq!(part2("01245"), "5");
    assert_eq!(part2("92510"), "18");
    assert_eq!(part2("59414"), "2018");
}
