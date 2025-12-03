// Greedily choose the largest battery that still leaves room for the number of batteries remaining.
// Optimal because if two numbers differ in a given digit, all subsequent digits are irrelevant.
fn max_jolts(mut bank: &str, mut batteries: usize) -> u64 {
    let mut jolts = String::new();
    while batteries > 0 {
        batteries -= 1;
        let (i, c) = bank[..bank.len() - batteries]
            .char_indices()
            .rev() // Reverse because we want the leftmost max and max_by_key returns the rightmost.
            .max_by_key(|&(_, c)| c)
            .unwrap();
        bank = &bank[i + 1..];
        jolts.push(c);
    }
    jolts.parse().unwrap()
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(|bank| max_jolts(bank, 2)).sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines().map(|bank| max_jolts(bank, 12)).sum()
}

pub fn tests() {
    let example = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    assert_eq!(part1(example), 357);
    assert_eq!(part2(example), 3_121_910_778_619);
}
