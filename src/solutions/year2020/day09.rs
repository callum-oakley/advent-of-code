use std::cmp::Ordering;

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn valid(window_size: usize, numbers: &[u64], i: usize) -> bool {
    (i - window_size + 1..i)
        .any(|j| (i - window_size..j).any(|k| numbers[i] == numbers[j] + numbers[k]))
}

fn part1_(window_size: usize, numbers: &[u64]) -> u64 {
    numbers[(window_size..numbers.len())
        .find(|&i| !valid(window_size, numbers, i))
        .unwrap()]
}

fn part2_(window_size: usize, numbers: &[u64]) -> u64 {
    let target = part1_(window_size, numbers);
    let mut i = 0;
    let mut j = 1;
    loop {
        match numbers[i..j].iter().sum::<u64>().cmp(&target) {
            Ordering::Less => {
                j += 1;
            }
            Ordering::Greater => {
                i += 1;
            }
            Ordering::Equal => {
                return numbers[i..j].iter().min().unwrap() + numbers[i..j].iter().max().unwrap();
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    part1_(25, &parse(input))
}

pub fn part2(input: &str) -> u64 {
    part2_(25, &parse(input))
}

pub fn tests() {
    let example = "35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576";
    assert_eq!(part1_(5, &parse(example)), 127);
    assert_eq!(part2_(5, &parse(example)), 62);
}
