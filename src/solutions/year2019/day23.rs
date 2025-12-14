use std::collections::VecDeque;

use crate::intcode::{State, VM};

#[derive(Clone)]
struct Computer {
    vm: VM,
    inbox: VecDeque<(i64, i64)>,
}

struct Network {
    computers: Vec<Computer>,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut computers = vec![
            Computer {
                vm: VM::new(input),
                inbox: VecDeque::new()
            };
            50
        ];
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.vm.input(i.try_into().unwrap());
        }
        Network { computers }
    }

    fn rcv(&mut self, i: usize) -> bool {
        if let Some((x, y)) = self.computers[i].inbox.pop_front() {
            self.computers[i].vm.input(x);
            self.computers[i].vm.input(y);
            true
        } else {
            self.computers[i].vm.input(-1);
            false
        }
    }

    fn snd(&mut self, i: usize) -> Option<(i64, i64)> {
        let dst: usize = self.computers[i].vm.output().try_into().unwrap();
        let packet = (self.computers[i].vm.output(), self.computers[i].vm.output());
        if dst == 255 {
            Some(packet)
        } else {
            self.computers[dst].inbox.push_back(packet);
            None
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut network = Network::new(input);
    loop {
        for i in 0..network.computers.len() {
            match network.computers[i].vm.state() {
                State::Input => {
                    network.rcv(i);
                }
                State::Output => {
                    if let Some(packet) = network.snd(i) {
                        return packet.1;
                    }
                }
                State::Halt => unreachable!(),
            }
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let mut network = Network::new(input);
    let mut nat = None;
    let mut last_y = None;
    loop {
        let mut idle = true;
        for i in 0..network.computers.len() {
            match network.computers[i].vm.state() {
                State::Input => {
                    if network.rcv(i) {
                        idle = false;
                    }
                }
                State::Output => {
                    idle = false;
                    if let Some(packet) = network.snd(i) {
                        nat = Some(packet);
                    }
                }
                State::Halt => unreachable!(),
            }
        }
        if idle && let Some(nat) = nat {
            if let Some(last_y) = last_y
                && nat.1 == last_y
            {
                return last_y;
            }
            last_y = Some(nat.1);
            network.computers[0].inbox.push_back(nat);
        }
    }
}
