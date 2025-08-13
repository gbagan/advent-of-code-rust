use crate::year2019::intcode::*;
use Status::*;

pub fn solve(input: &str) -> (i64, i64) {
    let machine = IntCode::with_extra_capacity(input, 100);

    let mut left50 = 0;

    while !test(&machine, left50 + 1, 50) {
        left50 += 1;
    }
    let mut right50 = left50;
    while test(&machine,right50 + 1, 50) {
        right50 += 1;
    }

    let input = Input { machine, left50, right50};

    let p1 = part1(&input);
    let p2 = part2(&input);

    (p1, p2)
}

fn part1(input: &Input) -> i64 {
    (0..50).map(|y| {
        let left = (0..50).find(|&x| input.test(x, y)).unwrap_or(i64::MAX);
        let right = (0..50).rev().find(|&x| input.test(x, y)).unwrap_or(i64::MIN);
        if left > right { 0} else { right - left + 1 }
    }).sum()
}

fn part2(input: &Input) -> i64 {
    let mut y = 5000 / (input.right50 - input.left50);
    let mut x = input.right50 * y / 50;
    
    loop {
        while input.test(x+1, y) {
            x += 1;
        }
        if input.test(x-99, y+99) {
            return 10_000 * (x - 99) + y
        }
        y += 1;
    }
}


fn test(machine: &IntCode, x: i64, y: i64) -> bool {
    let mut machine = machine.clone();
    machine.input(x);
    machine.input(y);
    match machine.run() {
        Output(1) => true,
        _ => false
    }
}

struct Input {
    machine: IntCode,
    left50: i64,
    right50: i64,
}

impl Input {
    fn test(&self, x: i64, y: i64) -> bool {
        if y == 0 {
            return x == 0;
        }
        self.left50 * y <= 50 * x && 50 * x < (self.right50 + 1) * y && test(&self.machine, x, y)
    }
}