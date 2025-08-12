use crate::year2019::intcode::*;

use Status::*;

pub fn solve(input: &str) -> (u32, i64) {
    let mut machine = IntCode::with_extra_capacity(input, 100);
    let mut machine2 = machine.clone();

    let p1 = part1(&mut machine);
    let p2 = part2(&mut machine2);

    (p1, p2)
}

fn part1(machine: &mut IntCode) -> u32 {
    let mut counter = 0u32;
    
    loop {
        let Output(_) = machine.run() else { break };
        let Output(_) = machine.run() else { break };
        let Output(tile) = machine.run() else { break };
        counter += (tile == 2) as u32;
    }

    counter
}

fn part2(machine: &mut IntCode) -> i64 {
    machine.set(0, 2);
    let mut ball = 0i64;
    let mut paddle = 0i64;
    let mut score = 0; 

    loop {
        match machine.run() {
            Halt => break,
            Input => machine.input((ball - paddle).signum()),
            Output(x) => {
                let Output(y) = machine.run() else { break };
                let Output(z) = machine.run() else { break };
                match (x, y, z) {
                    (-1, 0, _) => score = z,
                    (_, _, 3) => paddle = x,
                    (_, _, 4) =>  ball = x,
                    _ => {},
                }
            }
        }
    }

    score
}