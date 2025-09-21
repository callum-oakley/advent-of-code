fn parse(s: &str) -> i64 {
    let mut n = 0;
    let mut base = 1;
    for c in s.chars().rev() {
        n += match c {
            '-' => -1,
            '=' => -2,
            _ => i64::from(c.to_digit(10).unwrap()),
        } * base;
        base *= 5;
    }
    n
}

fn print(mut n: i64) -> String {
    let mut s = Vec::new();
    while n != 0 {
        match n % 5 {
            3 => {
                s.push('=');
                n = n / 5 + 1;
            }
            4 => {
                s.push('-');
                n = n / 5 + 1;
            }
            r => {
                s.push(char::from_digit(u32::try_from(r).unwrap(), 10).unwrap());
                n /= 5;
            }
        }
    }
    s.iter().rev().collect()
}

pub fn part1(input: &str) -> String {
    print(input.lines().map(parse).sum())
}

pub fn tests() {
    let example = "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122";
    assert_eq!(part1(example), "2=-1=0");
}
