use crate::util::parser::*;

pub fn solve(input: &str) -> (usize, usize) {
    let numbers: Vec<u32> = input.iter_unsigned().collect();
    let p1 = numbers.array_windows().filter(|[x, y]| x < y).count();
    let p2 = numbers.array_windows().filter(|[x, _, _, y]| x < y).count();

    (p1, p2)
}