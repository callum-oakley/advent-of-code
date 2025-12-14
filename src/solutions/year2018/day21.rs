use std::{collections::HashSet, iter};

use crate::{get_input, vm_2018};

// The program halts after the test on line 28 if r0 is equal to r5. r0 doesn't otherwise feature in
// the calculation, so by running the program and noting r5 each time we reach line 28, we produce a
// sequence of valid settings for r0 to cause the program to halt.
fn valid(input: &str) -> impl Iterator<Item = usize> {
    let (ip, instructions) = vm_2018::parse(input);
    let mut regs = [0; 6];
    iter::repeat_with(move || {
        loop {
            vm_2018::apply(instructions[regs[ip]], &mut regs);
            regs[ip] += 1;
            if regs[ip] == 28 {
                return regs[5];
            }
        }
    })
}

// Part 2 requires us to find the last element of the above sequence before a repeat, and this turns
// out to be fairly slow. Reimplementing the process gives a much needed speed boost. See
// corresponding line numbers in comments (lines 00-05 are the bitwise AND test).
#[rustfmt::skip]
fn valid_fast(_: &str) -> impl Iterator<Item = usize> {
    let mut regs = [0; 6];
    iter::repeat_with(move || {
        regs[2] = regs[5] | 65536;              // 06
        regs[5] = 7_571_367;                    // 07
        loop {                                  //
            regs[4] = regs[2] & 255;            // 08
            regs[5] += regs[4];                 // 09
            regs[5] &= 16_777_215;              // 10
            regs[5] *= 65899;                   // 11
            regs[5] &= 16_777_215;              // 12
            if regs[2] >= 256 {                 // 13-16
                regs[4] = 0;                    // 17
                loop {                          //
                    regs[3] = regs[4] + 1;      // 18
                    regs[3] *= 256;             // 19
                    if regs[2] >= regs[3] {     // 20-23
                        regs[4] += 1;           // 24
                        continue;               // 25
                    }                           //
                    regs[2] = regs[4];          // 26
                    break;                      // 27
                }                               //
            } else {                            //
                return regs[5]                  // 28-30
            }
        }
    })
}

pub fn part1(input: &str) -> usize {
    valid(input).next().unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut seen = HashSet::new();
    let mut res = 0;
    for v in valid_fast(input) {
        if seen.contains(&v) {
            break;
        }
        seen.insert(v);
        res = v;
    }
    res
}

pub fn tests() {
    let input = get_input(2018, 21);
    assert!(valid_fast(&input).take(10).eq(valid(&input).take(10)));
}

//        #ip 1
// 00     seti 123 0 5
// 01     bani 5 456 5
// 02     eqri 5 72 5
// 03     addr 5 1 1
// 04     seti 0 0 1
// 05     seti 0 9 5
// 06     bori 5 65536 2
// 07     seti 7571367 9 5
// 08     bani 2 255 4
// 09     addr 5 4 5
// 10     bani 5 16777215 5
// 11     muli 5 65899 5
// 12     bani 5 16777215 5
// 13     gtir 256 2 4
// 14     addr 4 1 1
// 15     addi 1 1 1
// 16     seti 27 1 1
// 17     seti 0 2 4
// 18     addi 4 1 3
// 19     muli 3 256 3
// 20     gtrr 3 2 3
// 21     addr 3 1 1
// 22     addi 1 1 1
// 23     seti 25 6 1
// 24     addi 4 1 4
// 25     seti 17 8 1
// 26     setr 4 6 2
// 27     seti 7 4 1
// 28     eqrr 5 0 4
// 29     addr 4 1 1
// 30     seti 5 5 1
