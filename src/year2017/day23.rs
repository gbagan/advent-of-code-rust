use itertools::Itertools;
use crate::util::parser::*;

pub fn solve(input: &str) -> (i32, usize) {
    let (x, _, _, _, y, z, t) = input.iter_signed::<i32>().next_tuple().unwrap();
    let p1 = (x-2) * (x-2);
    
    let b = x * y - z;
    let c = b - t;
    let p2 = (b..c+1).step_by(17).filter(|&x| !is_prime(x)).count();
    (p1, p2)
}

fn is_prime(n: i32) -> bool {
    n >= 2 && (2..(n as f64).sqrt() as i32).all(|i| n % i != 0)
}
