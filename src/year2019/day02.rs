use crate::year2019::intcode::*;

pub fn solve(input: &str) -> (usize, usize) {
    let mut machine = IntCode::new(input);
    let mut machine2 = machine.clone();
    let b = run(&mut machine, 0, 0);
    let a = run(&mut machine2, 1, 0) - b;

    let p1 = 12 * a + b + 2;
    let n = 19690720 - b;
	let noun = n / a;
	let verb = n - (noun * a);
    let p2 = 100 * noun + verb;

    (p1, p2)
}

fn run(machine: &mut IntCode, x: usize, y: usize) -> usize {
    machine.set(1, x);
    machine.set(2, y);
    machine.run();
    machine.get(0)

}