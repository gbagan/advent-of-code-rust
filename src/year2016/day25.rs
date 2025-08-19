use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (u32, u32) {
    let (c, b) = input.iter_unsigned::<u32>().next_tuple().unwrap();
    let m = c * b;
    
    let mut signal = 2;
    while signal < m {
        signal = signal << 2 | 2;
    }
    
    let p1 = signal - m;

    (p1, 0)
}