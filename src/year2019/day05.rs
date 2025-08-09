use crate::year2019::intcode::*;

pub fn solve(input: &str) -> (i32, i32) {
    let mut machine = IntCode::new(input);
    let mut machine2 = machine.clone();
    
    machine.input(1);
    let mut p1 = 0;
    while let Status::Output(val) = machine.run() {
        p1 = val;
    }

    machine2.input(5);
    let mut p2 = 0;
    while let Status::Output(val) = machine2.run() {
        p2 = val;
    }

    (p1, p2)
}