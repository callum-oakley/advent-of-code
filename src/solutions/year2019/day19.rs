use crate::{
    grid::{Vector, Z},
    intcode::VM,
};

fn in_beam(mut vm: VM, pos: Vector) -> bool {
    vm.input(pos.x);
    vm.input(pos.y);
    let res = vm.output() == 1;
    vm.halt();
    res
}

pub fn part1(input: &str) -> usize {
    let vm = VM::new(input);
    let mut res = 0;
    for y in 0..50 {
        for x in 0..50 {
            if in_beam(vm.clone(), Vector::new(x, y)) {
                res += 1;
            }
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let vm = VM::new(input);
    let mut pos = Z;
    loop {
        if !in_beam(vm.clone(), pos + Vector::new(99, 0)) {
            pos.y += 1;
        } else if !in_beam(vm.clone(), pos + Vector::new(0, 99)) {
            pos.x += 1;
        } else {
            return 10000 * pos.x + pos.y;
        }
    }
}
