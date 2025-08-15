use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (usize, usize) {
    let numbers: Vec<_> = input.iter_unsigned().collect();
    let p1 = count_triangles(numbers.iter().copied());
    let first = count_triangles(numbers.iter().step_by(3).copied());
    let second = count_triangles(numbers.iter().skip(1).step_by(3).copied());
    let third = count_triangles(numbers.iter().skip(2).step_by(3).copied());
    let p2 = first + second + third;
    (p1, p2)
}

fn count_triangles(iter: impl Iterator<Item = u32>) -> usize {
    iter.tuples().filter(|&(a, b, c)| a < b + c && b < a + c && c < a + b).count()
}