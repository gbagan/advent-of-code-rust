use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (usize, u32) {
    let p1 = input
        .lines()
        .skip(30)
        .filter(|&line| {
            let mut it = line.iter_unsigned::<u32>();
            let (x, y) = it.next_tuple().unwrap();
            (x / 3) * (y / 3) >= it.sum() 
        }).count();
    (p1, 0)
}