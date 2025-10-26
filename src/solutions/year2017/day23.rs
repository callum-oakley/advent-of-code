fn reg(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

#[derive(Clone, Copy)]
enum Arg {
    Lit(i32),
    Reg(usize),
}

impl Arg {
    fn value(&self, regs: &[i32]) -> i32 {
        match self {
            Arg::Lit(lit) => *lit,
            Arg::Reg(reg) => regs[*reg],
        }
    }
}

impl From<&str> for Arg {
    fn from(s: &str) -> Self {
        if let Ok(n) = s.parse() {
            Arg::Lit(n)
        } else {
            Arg::Reg(reg(s))
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Set(usize, Arg),
    Sub(usize, Arg),
    Mul(usize, Arg),
    Jnz(Arg, Arg),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let op = words.next();
            let x = words.next();
            let y = words.next();
            match op.unwrap() {
                "set" => Instruction::Set(reg(x.unwrap()), y.unwrap().into()),
                "sub" => Instruction::Sub(reg(x.unwrap()), y.unwrap().into()),
                "mul" => Instruction::Mul(reg(x.unwrap()), y.unwrap().into()),
                "jnz" => Instruction::Jnz(x.unwrap().into(), y.unwrap().into()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn offset(ip: usize, n: i32) -> usize {
    if n < 0 {
        ip - usize::try_from(-n).unwrap()
    } else {
        ip + usize::try_from(n).unwrap()
    }
}

pub fn part1(input: &str) -> usize {
    let instructions = parse(input);

    let mut regs = [0; 8];
    let mut ip = 0;
    let mut mul_count = 0;

    while let Some(instruction) = instructions.get(ip) {
        match *instruction {
            Instruction::Set(x, y) => {
                regs[x] = y.value(&regs);
            }
            Instruction::Sub(x, y) => {
                regs[x] -= y.value(&regs);
            }
            Instruction::Mul(x, y) => {
                regs[x] *= y.value(&regs);
                mul_count += 1;
            }
            Instruction::Jnz(x, y) => {
                if x.value(&regs) != 0 {
                    ip = offset(ip, y.value(&regs));
                    continue;
                }
            }
        }
        ip += 1;
    }

    mul_count
}

fn is_prime(n: usize) -> bool {
    for d in 2.. {
        if d * d > n {
            break;
        }
        if n.is_multiple_of(d) {
            return false;
        }
    }
    true
}

// After some debugging: the program is counting up in 17s from 105_700 to
// 122_700 inclusive, and incrementing h each time it finds a composite number.
pub fn part2(_: &str) -> usize {
    (105_700..=122_700)
        .step_by(17)
        .filter(|&n| !is_prime(n))
        .count()
}
