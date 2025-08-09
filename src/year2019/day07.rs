use crate::{util::foreach_permutation, year2019::intcode::*};

pub fn solve(input: &str) -> (i32, i32) {
    let machine = IntCode::new(input);
    
    let p1 = part1(&machine);
    let p2 = part2(&machine);

    (p1, p2)
}

fn part1(machine: &IntCode) -> i32 {
    let mut highest_signal = 0;
    let mut machine = machine.clone();
    foreach_permutation(&mut [0, 1, 2, 3, 4], |sequence| {
        let mut signal = 0;
        for &setting in sequence {
            machine.input(setting);
            machine.input(signal);
            match machine.run() {
                Status::Output(next) => signal = next,
                _ => panic!("expect an output"),
            }
            machine.reset();
        }
        highest_signal = highest_signal.max(signal);
    });
    highest_signal
}

fn part2(machine: &IntCode) -> i32 {
    let mut highest_signal = 0;
    let mut machines: [IntCode; 5] = std::array::from_fn(|_| machine.clone());
    
    foreach_permutation(&mut [5, 6, 7, 8, 9], |sequence| {
        let mut signal = 0;
        for (machine, &setting) in machines.iter_mut().zip(sequence) {
            machine.reset();
            machine.input(setting);
        }
        
        'outer: loop {
            for machine in &mut machines {
                machine.input(signal);
                match machine.run() {
                    Status::Output(next) => signal = next,
                    _ => break 'outer,
                }
            }
        }
        highest_signal = highest_signal.max(signal);
    });
    highest_signal
}