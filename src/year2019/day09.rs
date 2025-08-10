use crate::year2019::intcode::*;

pub fn solve(input: &str) -> (i64, i64) {
    let mut machine = IntCode::new(input);
    let mut machine2 = machine.clone();

    machine.input(1);
    let mut p1 = 0;
    while let Status::Output(val) = machine.run() {
        p1 = val;
    }

    //machine.reset();
    machine2.input(2);
    let mut p2 = 0;
    while let Status::Output(val) = machine2.run() {
        p2 = val;
    }

    (p1, p2)
}